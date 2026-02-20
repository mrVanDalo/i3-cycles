# NixOS Daemon Integration Tests
# Covers DAEM-01 through DAEM-04 requirements

{
  testers,
  i3-cycles,
  pkgs,
  ...
}:

testers.runNixOSTest {
  name = "i3-cycle-daemon-integration";

  nodes.machine =
    { pkgs, ... }:
    {
      environment.systemPackages = [
        i3-cycles
        pkgs.socat
        pkgs.jq
        pkgs.procps
      ];

      users.users.testuser = {
        isNormalUser = true;
        uid = 1000;
        home = "/home/testuser";
      };

      systemd.services.i3-cycle-daemon = {
        description = "i3-cycle daemon";
        after = [ "multi-user.target" ];
        serviceConfig = {
          Type = "simple";
          ExecStart = "${i3-cycles}/bin/i3-cycle-daemon";
          Restart = "on-failure";
          User = "testuser";
          Environment = "XDG_RUNTIME_DIR=/tmp";
          StandardOutput = "journal";
          StandardError = "journal";
        };
        wantedBy = [ "multi-user.target" ];
      };
    };

  testScript = ''
    import json
    import time

    machine.wait_for_unit("multi-user.target")
    machine.wait_for_unit("i3-cycle-daemon.service")
    time.sleep(1)

    def send_request(request_dict):
        request_json = json.dumps(request_dict)
        machine.succeed("echo '%s' > /tmp/request.json" % request_json)
        result = machine.succeed("su - testuser -c 'cat /tmp/request.json | ${pkgs.socat}/bin/socat -t5 - UNIX-CONNECT:/tmp/i3-cycles.sock'").strip()
        return json.loads(result) if result else None

    def daemon_logs():
        return machine.succeed("journalctl -u i3-cycle-daemon --no-pager -n 50")

    # DAEM-01: Multiple Sequential Requests
    machine.log("=== Testing DAEM-01: Multiple Sequential Requests ===")

    response = send_request({"SelectCycle": {"name": "test_cycle"}})
    assert response == {"CycleSelected": {"action": "Created"}}, "DAEM-01: Unexpected response: " + str(response)

    for ws in ["ws1", "ws2", "ws3"]:
        response = send_request({"Toggle": {"workspace": ws}})
        assert response == {"Toggled": {"action": "Added"}}, "DAEM-01: Failed to add workspace: " + str(response)

    response = send_request("ListCycles")
    assert "test_cycle" in response["Cycles"]["names"], "DAEM-01: test_cycle not found: " + str(response)

    workspaces_seen = []
    for i in range(6):
        response = send_request("Next")
        ws = response["NextWorkspace"]["workspace"]
        workspaces_seen.append(ws)
        machine.log("DAEM-01: next() returned: " + ws)

    expected = ["ws1", "ws2", "ws3", "ws1", "ws2", "ws3"]
    assert workspaces_seen == expected, "DAEM-01: Expected " + str(expected) + ", got " + str(workspaces_seen)
    machine.log("DAEM-01 PASSED")

    # DAEM-02: State Persistence Across Client Connections
    machine.log("=== Testing DAEM-02: State Persistence ===")

    response = send_request({"SelectCycle": {"name": "persist_cycle"}})
    assert response == {"CycleSelected": {"action": "Created"}}, "DAEM-02: Failed to create cycle: " + str(response)

    for ws in ["a", "b", "c"]:
        response = send_request({"Toggle": {"workspace": ws}})
        assert response == {"Toggled": {"action": "Added"}}, "DAEM-02: Failed to add workspace: " + str(response)

    response = send_request("Next")
    assert response["NextWorkspace"]["workspace"] == "a", "DAEM-02: Expected 'a', got: " + str(response)

    response = send_request("Next")
    assert response["NextWorkspace"]["workspace"] == "b", "DAEM-02: Expected 'b', got: " + str(response)

    # After 2 next() calls with 3 workspaces, position = (0 + 2) % 3 = 2
    response = send_request("Status")
    status = json.loads(response["Status"]["json"])
    cycle_data = status["cycle_data"]["persist_cycle"]
    assert cycle_data["position"] == 2, "DAEM-02: Expected position 2, got %d" % cycle_data["position"]
    machine.log("DAEM-02 PASSED")

    # DAEM-03: Concurrent Client Requests
    machine.log("=== Testing DAEM-03: Concurrent Requests ===")

    response = send_request({"SelectCycle": {"name": "concurrent_cycle"}})
    assert response == {"CycleSelected": {"action": "Created"}}, "DAEM-03: Failed to create cycle: " + str(response)

    for ws in ["x", "y", "z"]:
        send_request({"Toggle": {"workspace": ws}})

    # Create request files
    requests = []
    for i in range(9):
        request = {"SelectCycle": {"name": "concurrent_cycle"}} if i == 0 else "Next"
        requests.append(json.dumps(request))

    for i, req in enumerate(requests):
        machine.succeed("echo '%s' > /tmp/req_%d.json" % (req, i))

    # Send concurrent requests
    script = ""
    for i in range(9):
        script += "cat /tmp/req_%d.json | ${pkgs.socat}/bin/socat -t5 - UNIX-CONNECT:/tmp/i3-cycles.sock > /tmp/resp_%d.txt &\n" % (i, i)
    script += "wait\n"
    machine.succeed("su - testuser -c '%s'" % script)
    time.sleep(2)

    responses = []
    for i in range(9):
        try:
            result = machine.succeed("cat /tmp/resp_%d.txt" % i).strip()
            if result:
                responses.append(json.loads(result))
        except:
            pass

    assert len(responses) > 0, "DAEM-03: No responses received"

    next_responses = [r for r in responses if "NextWorkspace" in r]
    for resp in next_responses:
        ws = resp["NextWorkspace"]["workspace"]
        assert ws in ["x", "y", "z", "back_and_forth"], "DAEM-03: Invalid workspace: " + str(ws)

    machine.log("DAEM-03 PASSED: Processed %d requests" % len(responses))

    # DAEM-04: Daemon Restart Clears State
    machine.log("=== Testing DAEM-04: Daemon Restart ===")

    response = send_request({"SelectCycle": {"name": "restart_cycle"}})
    assert response == {"CycleSelected": {"action": "Created"}}, "DAEM-04: Failed to create cycle: " + str(response)

    for ws in ["r1", "r2", "r3"]:
        send_request({"Toggle": {"workspace": ws}})

    send_request("Next")
    send_request("Next")

    response = send_request("Status")
    status = json.loads(response["Status"]["json"])
    cycle_data = status["cycle_data"]["restart_cycle"]
    assert cycle_data["position"] == 2, "DAEM-04: Expected position 2"

    machine.systemctl("stop i3-cycle-daemon.service")
    time.sleep(2)
    machine.fail("systemctl is-active i3-cycle-daemon.service")
    machine.log("DAEM-04: Daemon stopped")

    machine.systemctl("start i3-cycle-daemon.service")
    time.sleep(2)
    machine.wait_for_unit("i3-cycle-daemon.service")
    time.sleep(1)
    machine.log("DAEM-04: Daemon restarted")

    response = send_request("Status")
    status = json.loads(response["Status"]["json"])

    if "restart_cycle" in status.get("cycle_data", {}):
        cycle_data = status["cycle_data"]["restart_cycle"]
        assert cycle_data["position"] == 0, "DAEM-04: Expected position 0, got %d" % cycle_data["position"]

    machine.log("DAEM-04 PASSED")

    # Logging Verification
    machine.log("=== Verifying Logging ===")
    logs = daemon_logs()
    assert "i3-cycle-daemon starting" in logs, "Logging: Daemon start not found"
    assert "Listening on" in logs, "Logging: Socket path not found"
    assert "Client connected" in logs or "Next workspace" in logs, "Logging: Operation logs not found"
    machine.log("Logging PASSED")

    machine.log("\n=== ALL DAEM TESTS PASSED ===")
  '';
}
