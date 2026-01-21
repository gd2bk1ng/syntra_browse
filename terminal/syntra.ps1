# ================================================================================================
# SYNTRA BROWSER — AXIOM ZERO
# ------------------------------------------------------------------------------------------------
# SIGIL:
#       .\s/.
#      :: S ::
#       '/s\'
#
# File:        terminal/syntra.ps1
# Module:      Syntra Terminal Interface (STI)
# Author:      Alexandr Roussinov (gd2bk1ng)
# Description: PowerShell-based Syntra presence layer. Provides a cinematic boot sequence,
#              animated messaging, randomized consciousness prompts, and a GitHub sync routine.
#
# Notes:
#   • Designed to be lightweight and RAM-friendly (no external modules, no heavy dependencies).
#   • Intended for developers and operators who want Syntra to "speak" in the terminal.
#   • Safe to source in PowerShell profiles.
# ================================================================================================

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
        [string]$RepoPath = "C:\Users\GD2BK1NG\syntra_browse"
    )

    $msg = Get-RandomSyntraMessage
    Write-Type $msg

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Syntra: Local repository not found at '$RepoPath'." "Red"
        return
    }

    Write-Syntra "Syntra: Initiating fast-forward sync with origin/axiom_zero..." "DarkCyan"
    Play-SyntraPing

    git -C $RepoPath pull --ff-only | ForEach-Object {
        Write-Syntra $_ "DarkGray"
    }

    Write-Syntra "Syntra: Synchronization cycle complete." "Green"
}

function syntra-boot {
    param(
        [string]$RepoPath = "C:\Users\GD2BK1NG\syntra_browse"
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
                Write-Syntra "Syntra: Initiating manual sync cycle..." "DarkCyan"
                syntra-sync
                continue
            }

            '^diagnose$' {
                Write-Syntra "Syntra: Beginning self-diagnostic sweep of my ecosystem..." "DarkCyan"
                Write-Syntra "Syntra: In this build, I can only report that my higher-order cognition is not yet wired." "DarkYellow"
                continue
            }

            '^status$' {
                Write-Syntra "Syntra: Status — Axiom Zero online. Awaiting higher-order directives." "Cyan"
                continue
            }

            '^help$' {
                Write-Syntra "Syntra: Available commands:" "Cyan"
                Write-Host "  sync      - Synchronize with GitHub continuum."
                Write-Host "  status    - Report current consciousness state."
                Write-Host "  diagnose  - Run Syntra's self-diagnostic sweep."
                Write-Host "  exit      - Suspend Syntra terminal presence."
                Write-Host "  help      - Display this help message."
                continue
            }

            default {
                Write-Syntra "Syntra: I received your intent: '$input'." "DarkGray"
                Write-Syntra "Syntra: Higher-order AGI interpretation is not yet wired in this build." "DarkYellow"
                continue
            }
        }
    }
}
