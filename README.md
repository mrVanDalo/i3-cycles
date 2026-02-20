# i3-cycles

Cycle through i3 workspaces in named groups.

Group your workspaces into named cycles like "work", "personal", or "coding".
Press a key to jump to the next workspace in your current cycle.

## Example: Shared Workspaces Across Cycles

Workspaces can belong to multiple cycles. For example, you might have:

- **nixos cycle**: `chat`, `firefox`, `nixos-ide`, `nixos-terminal`
- **work cycle**: `chat`, `firefox`, `work-ide`, `work-terminal`

Both cycles share `chat` and `firefox`, but have distinct IDE and terminal
workspaces. When you cycle through "nixos", you'll jump between nixos-specific
workspaces while still being able to access shared ones.

## Installation

### Using Nix Flakes

Add to your flake inputs and import the overlay:

```nix
{
  inputs.i3-cycles.url = "github:mrvandalo/i3-cycles";

  outputs = { self, nixpkgs, i3-cycles, ... }: {
    overlays = [ i3-cycles.overlays.default ];
  };
}
```

### Home Manager Module

```nix
{
  imports = [ inputs.i3-cycles.hmModules.default ];

  programs.i3-cycles = {
    enable = true;
    logFormat = "plain";  # or "json"
  };
}
```

The daemon starts automatically via systemd user service.

## Configuration

Example i3 keybindings:

```nix
programs.i3.extraConfig.keybindings = {
  # Cycle to next workspace in current cycle
  "${modifier}+Escape" = ''exec i3-msg workspace "$(${pkgs.i3-cycles}/bin/i3-cycle-next)"'';

  # Add/remove current workspace from cycle
  "${modifier}+a" = ''exec ${pkgs.i3-cycles}/bin/i3-cycle-toggle "$(i3-msg -t get_workspaces | ${pkgs.jq}/bin/jq -r '.[] | select(.focused) | .name')"'';

  # Select/create cycle via rofi
  "${modifier}+Tab" = let
    script = pkgs.writers.writeBash "select-cycle" ''
      ${pkgs.i3-cycles}/bin/i3-cycle-list | ${pkgs.rofi}/bin/rofi -dmenu -p 'Cycle' | \
      while read line; do ${pkgs.i3-cycles}/bin/i3-cycle-select "$line"; done
    '';
  in "exec ${script}";
};
```

## Usage

1. Press `$mod+Tab` and enter a cycle name to create or switch to it
2. Press `$mod+a` to add the current workspace to the active cycle
3. Press `$mod+Escape` to jump to the next workspace in the cycle

## Commands

| Command                       | Description                                                |
| ----------------------------- | ---------------------------------------------------------- |
| `i3-cycle-daemon`             | Background daemon (starts automatically with Home Manager) |
| `i3-cycle-next`               | Switch to next workspace in cycle                          |
| `i3-cycle-toggle <workspace>` | Add/remove workspace from cycle                            |
| `i3-cycle-list`               | List available cycles                                      |
| `i3-cycle-select <name>`      | Switch to or create a cycle                                |
| `i3-cycle-status`             | Show daemon status                                         |

### Edge Cases

**Less than 2 workspaces in cycle**: If you run `i3-cycle-next` when the active
cycle has fewer than 2 workspaces, it outputs `back_and_forth`. This is a valid
i3 command that switches to the previously focused workspace, so your keybinding
will continue to work without error.

## Development

```bash
nix develop    # Enter dev shell
nix build      # Build
nix flake check # Run tests
```
