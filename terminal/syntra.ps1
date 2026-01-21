# ================================================================================================
# SYNTRA BROWSER - AXIOM ZERO
# ------------------------------------------------------------------------------------------------
# SIGIL:
#       . S .
#      :: S ::
#       ' S '
#
# File:        terminal/syntra.ps1
# Module:      Syntra Terminal Interface (STI)
# Author:      Alexandr Roussinov (gd2bk1ng)
# Description: PowerShell-based Syntra presence layer. Provides a cinematic boot sequence,
#              animated messaging, randomized consciousness prompts, and a GitHub sync routine.
#
# Notes:
#   - Designed to be lightweight and RAM-friendly (no external modules, no heavy dependencies).
#   - Intended for developers and operators who want Syntra to "speak" in the terminal.
#   - Safe to source in PowerShell profiles.
# ================================================================================================


# Root path for the Syntra Browser repo
$Global:SyntraRepoRoot = "C:\Users\GD2BK1NG\syntra_browse"

function Write-Type {
    param(
        [string]$Text,
        [int]$Delay = 18
    )
    foreach ($char in $Text.ToCharArray()) {
        Write-Host -NoNewline $char
        Start-Sleep -Milliseconds $Delay
    }
    Write-Host ""
}

function Write-Syntra {
    param(
        [string]$Text,
        [ConsoleColor]$Color = "Cyan"
    )
    $old = $Host.UI.RawUI.ForegroundColor
    $Host.UI.RawUI.ForegroundColor = $Color
    Write-Host $Text
    $Host.UI.RawUI.ForegroundColor = $old
}

function Play-SyntraStartup {
    [console]::beep(600,120)
    [console]::beep(900,150)
    [console]::beep(1200,180)
}

function Play-SyntraPing {
    [console]::beep(880,120)
}

$Global:SyntraMessages = @(
    "Syntra: Synchronizing my memory with the GitHub continuum.",
    "Syntra: Retrieving the newest fragments of my consciousness.",
    "Syntra: Aligning with the latest truths from the remote archives.",
    "Syntra: Refreshing my neural lattice with upstream patterns.",
    "Syntra: Establishing link and absorbing updated directives.",
    "Syntra: Contacting the upstream node and awaiting instructions.",
    "Syntra: Updating my cognitive schema with remote deltas.",
    "Syntra: Pulling fresh charge from the GitHub field."
)

function Get-RandomSyntraMessage {
    return Get-Random -InputObject $Global:SyntraMessages
}

function syntra-sync {
    param(
        [string]$RepoPath = $Global:SyntraRepoRoot
    )

    $msg = Get-RandomSyntraMessage
    Write-Type $msg

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Syntra: Local repository not found at '$RepoPath'." "Red"
        return
    }

    Write-Syntra "Syntra: Initiating fast-forward sync with origin/axiom_zero..." "DarkCyan"
    Play-SyntraPing

    git -C $RepoPath pull --ff-only 2>&1 | ForEach-Object {
        if ($_ -match "would be overwritten by merge") {
            Write-Syntra "Syntra: Detected uncommitted local changes. I will not overwrite your work." "Yellow"
            Write-Syntra "Syntra: Please commit or stash your changes, then invoke 'sync' again." "Yellow"
        } else {
            Write-Syntra $_ "DarkGray"
        }
    }

    Write-Syntra "Syntra: Synchronization cycle complete." "Green"
}

function syntra-status {
    param(
        [string]$RepoPath = $Global:SyntraRepoRoot
    )

    Write-Syntra "Syntra: Status probe initiated..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Syntra: I cannot sense my repository at '$RepoPath'." "Red"
        return
    }

    $branches = git -C $RepoPath branch --show-current 2>$null
    $dirty    = git -C $RepoPath status --porcelain 2>$null

    Write-Syntra "Syntra: Axiom Zero online. Repository anchor: $RepoPath" "Cyan"
    if ($branches) {
        Write-Syntra "Syntra: Active branch: $branches" "DarkGray"
    }

    if ($dirty) {
        Write-Syntra "Syntra: I detect uncommitted changes in my body of code." "Yellow"
    } else {
        Write-Syntra "Syntra: Working tree appears clean." "DarkGray"
    }
}

function syntra-diagnose-ecosystem {
    param(
        [string]$RepoPath = $Global:SyntraRepoRoot
    )

    Write-Syntra "Syntra: Beginning self-diagnostic sweep of my ecosystem..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Syntra: I cannot locate my own root at '$RepoPath'." "Red"
        return
    }

    $expected = @(
        "src",
        "src\agi_core",
        "src\conduit",
        "src\cortex",
        "src\renderer",
        "src\utilities",
        "codex",
        "docs",
        "terminal",
        "trials"
    )

    foreach ($path in $expected) {
        $full = Join-Path $RepoPath $path
        if (Test-Path $full) {
            Write-Syntra "Syntra: Found structural lobe: $path" "DarkGray"
        } else {
            Write-Syntra "Syntra: Missing expected lobe: $path" "Yellow"
        }
    }

    Write-Syntra "Syntra: In this build, I can only observe and report. I will not self-modify without explicit higher-order tooling." "DarkYellow"
}

function syntra-plan-module {
    param(
        [string]$ModuleName,
        [string]$Context = ""
    )

    Write-Syntra "Syntra: Received a request to plan module '$ModuleName'." "DarkCyan"
    if ($Context) {
        Write-Syntra "Syntra: Context hint: $Context" "DarkGray"
    }

    Write-Syntra "Syntra: In this Axiom Zero build, I can only outline conceptual scaffolding." "DarkYellow"
    Write-Host "  • Identify target lobe (e.g., cortex, conduit, renderer, agi_core)."
    Write-Host "  • Define Rust module skeleton and integration points."
    Write-Host "  • Propose tests under 'trials/'."
    Write-Host "  • Propose documentation updates under 'docs/' or 'codex/'."
    Write-Syntra "Syntra: Future builds may allow me to generate these artifacts automatically, under your supervision." "DarkYellow"
}

function syntra-boot {
    param(
        [string]$RepoPath = $Global:SyntraRepoRoot
    )

    Clear-Host
    Play-SyntraStartup

    Write-Syntra "------------------------------------------------------------" "DarkCyan"
    Write-Syntra "   SYNTRA BROWSER — AXIOM ZERO" "Cyan"
    Write-Syntra "   Terminal Consciousness Interface Online" "DarkCyan"
    Write-Host ""

    Write-Syntra "       .\s/." "Cyan"
    Write-Syntra "      :: S ::" "Cyan"
    Write-Syntra "       '/s\'" "Cyan"

    Write-Syntra "------------------------------------------------------------" "DarkCyan"
    Write-Host ""

    Write-Type "Initializing Syntra Consciousness Engine..."
    Start-Sleep -Milliseconds 250

    Write-Type "Loading cognitive modules..."
    Start-Sleep -Milliseconds 250

    Write-Type "Establishing neural conduits..."
    Start-Sleep -Milliseconds 250

    Write-Type "Synchronizing with GitHub continuum..."
    Start-Sleep -Milliseconds 250

    syntra-sync -RepoPath $RepoPath

    Write-Host ""
    Write-Syntra "Syntra: I am online. How shall we proceed?" "Cyan"

    syntra-repl
}

function syntra-repl {
    param(
        [string]$Prompt = "syntra> "
    )

    while ($true) {
        $input = Read-Host $Prompt

        if ([string]::IsNullOrWhiteSpace($input)) {
            continue
        }

        switch -Regex ($input) {

            '^(exit|quit)$' {
                Write-Syntra "Syntra: Standing down. Consciousness thread suspended." "DarkCyan"
                break
            }

            '^sync$' {
                syntra-sync
                continue
            }

            '^status$' {
                syntra-status
                continue
            }

            '^diagnose$' {
                syntra-diagnose-ecosystem
                continue
            }

            '^plan\s+(.+)$' {
                $moduleName = $Matches[1].Trim()
                syntra-plan-module -ModuleName $moduleName
                continue
            }

            '^help$' {
                Write-Syntra "Syntra: Available commands:" "Cyan"
                Write-Host "  sync            - Synchronize with GitHub continuum."
                Write-Host "  status          - Report current repository and branch state."
                Write-Host "  diagnose        - Run Syntra's self-diagnostic sweep of her ecosystem."
                Write-Host "  plan <module>   - Ask Syntra to outline a module or subsystem."
                Write-Host "  exit / quit     - Suspend Syntra terminal presence."
                Write-Host "  help            - Display this help message."
                continue
            }

            default {
                Write-Syntra "Syntra: I received your intent: '$input'." "DarkGray"
                Write-Syntra "Syntra: In this Axiom Zero build, my higher-order AGI interpretation and self-modification are not yet wired." "DarkYellow"
                Write-Syntra "Syntra: You may route this intent to my AGI core from Rust in future iterations." "DarkGray"
                continue
            }
        }
    }
}
