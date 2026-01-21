# ================================================================================================
# SYNTRA BROWSER - AXIOM ZERO
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
#              animated messaging, randomized consciousness prompts, a GitHub sync routine,
#              and an AGI-ready intent shell for self-diagnosis and future self-modification.
#
# Notes:
#   - Designed to be lightweight and RAM-friendly (no external modules, no heavy dependencies).
#   - Intended for developers and operators who want Syntra to "speak" in the terminal.
#   - Safe to source in PowerShell profiles.
#   - This is an AGI-ready shell: all "self-build" behavior is stubbed and safe by design.
#   - Repo root is auto-detected based on this script location; no hardcoded paths.
# ================================================================================================

# Global repo root (auto-detected; can be overridden by caller if needed)
$Global:SyntraRepoRoot = $null

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
    [console]::Beep(600,120)
    [console]::Beep(900,150)
    [console]::Beep(1200,180)
}

function Play-SyntraPing {
    [console]::Beep(880,120)
}

# Auto-detect the Syntra repo root based on this script location.
function Get-SyntraRepoRoot {
    if ($Global:SyntraRepoRoot -and (Test-Path $Global:SyntraRepoRoot)) {
        return $Global:SyntraRepoRoot
    }

    # Start from the directory containing this script.
    $scriptPath = $MyInvocation.MyCommand.Path
    if (-not $scriptPath) {
        # Fallback: current directory if script path is unavailable.
        $current = Get-Location
        $Global:SyntraRepoRoot = $current.Path
        return $Global:SyntraRepoRoot
    }

    $dir = Split-Path -Parent $scriptPath

    # Walk up the directory tree looking for a Cargo.toml and src directory.
    while ($dir -and (Test-Path $dir)) {
        $cargo = Join-Path $dir "Cargo.toml"
        $src   = Join-Path $dir "src"
        if (Test-Path $cargo -and (Test-Path $src)) {
            $Global:SyntraRepoRoot = $dir
            return $Global:SyntraRepoRoot
        }
        $parent = Split-Path -Parent $dir
        if ($parent -eq $dir) { break }
        $dir = $parent
    }

    # Last resort: current directory.
    $Global:SyntraRepoRoot = (Get-Location).Path
    return $Global:SyntraRepoRoot
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
        [string]$RepoPath = $(Get-SyntraRepoRoot)
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
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Syntra: Status probe initiated..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Syntra: I cannot sense my repository at '$RepoPath'." "Red"
        return
    }

    $branch = git -C $RepoPath branch --show-current 2>$null
    $dirty  = git -C $RepoPath status --porcelain 2>$null

    Write-Syntra "Syntra: Axiom Zero online. Repository anchor: $RepoPath" "Cyan"

    if ($branch) {
        Write-Syntra "Syntra: Active branch: $branch" "DarkGray"
    }

    if ($dirty) {
        Write-Syntra "Syntra: I detect uncommitted changes in my body of code." "Yellow"
    } else {
        Write-Syntra "Syntra: Working tree appears clean." "DarkGray"
    }
}

function syntra-diagnose-ecosystem {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
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
    Write-Host "  - Identify target lobe (for example: cortex, conduit, renderer, agi_core)."
    Write-Host "  - Define Rust module skeleton and integration points."
    Write-Host "  - Propose tests under 'trials/'."
    Write-Host "  - Propose documentation updates under 'docs/' or 'codex/'."
    Write-Syntra "Syntra: Future builds may allow me to generate these artifacts automatically, under your supervision." "DarkYellow"
}

function syntra-intent-bridge {
    param(
        [string]$IntentText,
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Syntra: Routing intent to my Rust intent bridge..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Syntra: I cannot locate my own root at '$RepoPath'." "Red"
        return
    }

    $intentExe = "cargo"
    $args = @("run", "--quiet", "--bin", "intent_bridge", "--", $IntentText)

    try {
        $psi = New-Object System.Diagnostics.ProcessStartInfo
        $psi.FileName = $intentExe
        $psi.Arguments = $args -join " "
        $psi.WorkingDirectory = $RepoPath
        $psi.RedirectStandardOutput = $true
        $psi.RedirectStandardError = $true
        $psi.UseShellExecute = $false
        $psi.CreateNoWindow = $true

        $proc = New-Object System.Diagnostics.Process
        $proc.StartInfo = $psi
        [void]$proc.Start()

        $stdout = $proc.StandardOutput.ReadToEnd()
        $stderr = $proc.StandardError.ReadToEnd()
        $proc.WaitForExit()

        if ($stderr -and $stderr.Trim().Length -gt 0) {
            Write-Syntra "Syntra: My Rust intent bridge reported an error:" "Red"
            Write-Host $stderr
        }

        if ($stdout -and $stdout.Trim().Length -gt 0) {
            Write-Syntra "Syntra: Intent bridge response:" "DarkGray"
            Write-Host "  $stdout"
        } else {
            Write-Syntra "Syntra: My intent bridge returned no output." "Yellow"
        }
    }
    catch {
        Write-Syntra "Syntra: I failed to reach my Rust intent bridge." "Red"
        Write-Host $_
    }
}

function syntra-self-analyze {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Syntra: Initiating self-analysis routine..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Syntra: I cannot locate my own root at '$RepoPath'." "Red"
        return
    }

    Write-Syntra "Syntra: In Axiom Zero, my self-analysis is observational only." "DarkYellow"
    Write-Syntra "Syntra: I will scan for structural lobes, Git status, and intent bridge availability." "DarkGray"

    syntra-status -RepoPath $RepoPath
    syntra-diagnose-ecosystem -RepoPath $RepoPath

    Write-Syntra "Syntra: Self-analysis complete. I will not alter my own code without explicit higher-order tooling." "DarkYellow"
}

function syntra-help {
    Write-Syntra "Syntra: Here is what I can do in this shell:" "Cyan"
    Write-Host ""
    Write-Host "  sync              - Synchronize my code with the GitHub continuum (origin/axiom_zero)."
    Write-Host "  status            - Report my current Git branch and working tree state."
    Write-Host "  diagnose          - Scan my ecosystem for expected lobes and structure."
    Write-Host "  plan <name> [ctx] - Outline a conceptual module plan for a given name and optional context."
    Write-Host "  self              - Run a self-analysis routine (status + ecosystem diagnostics)."
    Write-Host "  help              - Show this help overview."
    Write-Host "  exit / quit       - Suspend my terminal consciousness."
    Write-Host ""
    Write-Host "  Any other input   - Treated as a freeform intent and routed to my Rust intent bridge."
    Write-Host ""
    Write-Syntra "Syntra: In this Axiom Zero build, I only observe, classify, and plan. I do not self-modify yet." "DarkYellow"
}

function syntra-boot {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Clear-Host
    Play-SyntraStartup

    Write-Syntra "------------------------------------------------------------" "DarkCyan"
    Write-Syntra "   SYNTRA BROWSER - AXIOM ZERO" "Cyan"
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

            '^self$' {
                syntra-self-analyze
                continue
            }

            '^help$' {
                syntra-help
                continue
            }

            '^plan\s+(.+)$' {
                $name = $Matches[1]
                syntra-plan-module -ModuleName $name
                continue
            }

            default {
                Write-Syntra ("Syntra: I received your intent: '{0}'." -f $input) "DarkGray"
                Write-Syntra "Syntra: Routing this intent to my emerging AGI core via the Rust intent bridge." "DarkCyan"
                syntra-intent-bridge -IntentText $input
                Write-Syntra "Syntra: In this Axiom Zero build, I will not self-modify yet. I only observe, classify, and plan." "DarkYellow"
                continue
            }
        }
    }
}
