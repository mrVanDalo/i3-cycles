{
  testers,
  i3-cycles,
  ...
}:

testers.runNixOSTest {
  name = "i3-cycles";

  nodes.machine =
    { pkgs, ... }:
    {
      environment.systemPackages = [ i3-cycles ];
      users.users.testuser = {
        isNormalUser = true;
        uid = 1000;
      };
    };

  testScript = ''
    import time

    machine.wait_for_unit("multi-user.target")

    # Test 1: Start daemon and verify it's running
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp setsid -f ${i3-cycles}/bin/i3-cycle-daemon > /dev/null 2>&1'")
    time.sleep(1)

    # Test 2: Test i3-cycle-next with empty cycle (should return back_and_forth)
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-next'").strip()
    assert output == "back_and_forth", f"Expected 'back_and_forth', got '{output}'"

    # Test 3: Toggle workspaces into cycle
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-toggle workspace1'")
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-toggle workspace2'")
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-toggle workspace3'")

    # Test 4: Verify cycle next returns a workspace (not back_and_forth)
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-next'").strip()
    assert output in ["workspace1", "workspace2", "workspace3"], f"Expected workspace name, got '{output}'"

    # Test 5: List cycles
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-list'").strip()
    assert "default" in output, f"Expected 'default' cycle, got '{output}'"

    # Test 6: Create and select a new cycle
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-select work'")
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-list'").strip()
    assert "work" in output, f"Expected 'work' cycle, got '{output}'"

    # Test 7: Verify non-default cycle returns back_and_forth when empty
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-next'").strip()
    assert output == "back_and_forth", f"Expected 'back_and_forth' for empty 'work' cycle, got '{output}'"

    # Test 8: Add workspaces to non-default cycle
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-toggle work1'")
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-toggle work2'")

    # Test 9: Verify i3-cycle-next returns workspace from non-default cycle
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-next'").strip()
    assert output in ["work1", "work2"], f"Expected workspace from 'work' cycle, got '{output}'"

    # Test 10: Get status
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-status'")
    assert "active_cycle" in output, f"Expected JSON status, got '{output}'"

    # Test 11: Switch back to default cycle and verify correct workspaces
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-select default'")
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-next'").strip()
    assert output in ["workspace1", "workspace2", "workspace3"], f"Expected workspace from 'default' cycle, got '{output}'"

    # Test 12: Switch back to work cycle and verify correct workspaces
    machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-select work'")
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-next'").strip()
    assert output in ["work1", "work2"], f"Expected workspace from 'work' cycle, got '{output}'"

    # Test 13: Test fallback when daemon not running (kill daemon first)
    machine.succeed("pkill i3-cycle-daemon")
    time.sleep(1)
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-next'").strip()
    assert output == "back_and_forth", f"Expected fallback 'back_and_forth', got '{output}'"

    # Test 14: List should return empty when daemon not running
    output = machine.succeed("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-list'").strip()
    assert output == "", f"Expected empty output when daemon down, got '{output}'"

    # Test 15: Status should fail when daemon not running
    machine.fail("su - testuser -c 'XDG_RUNTIME_DIR=/tmp ${i3-cycles}/bin/i3-cycle-status'")
  '';
}
