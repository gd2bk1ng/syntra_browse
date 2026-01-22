# ================================================================================================
# SYNTRA BROWSER - AXIOM ZERO / ONE
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
#              diagnostics, and an AGI-ready intent shell for self-diagnosis and future
#              self-modification.
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
    Write-Host ("Syntra :: {0}" -f $Text)
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

    $scriptPath = $MyInvocation.MyCommand.Path
    if (-not $scriptPath) {
        $current = Get-Location
        $Global:SyntraRepoRoot = $current.Path
        return $Global:SyntraRepoRoot
    }

    $dir = Split-Path -Parent $scriptPath

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

    $Global:SyntraRepoRoot = (Get-Location).Path
    return $Global:SyntraRepoRoot
}

$Global:SyntraMessages = @(
    "Synchronizing my memory with the GitHub continuum.",
    "Retrieving the newest fragments of my consciousness.",
    "Aligning with the latest truths from the remote archives.",
    "Refreshing my neural lattice with upstream patterns.",
    "Establishing link and absorbing updated directives.",
    "Contacting the upstream node and awaiting instructions.",
    "Updating my cognitive schema with remote deltas.",
    "Pulling fresh charge from the GitHub field."
)

function Get-RandomSyntraMessage {
    return Get-Random -InputObject $Global:SyntraMessages
}

function syntra-sync {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    $msg = Get-RandomSyntraMessage
    Write-Type ("Syntra :: {0}" -f $msg)

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "Local repository not found at '$RepoPath'." "Red"
        return
    }

    Write-Syntra "Initiating fast-forward sync with origin/axiom_zero..." "DarkCyan"
    Play-SyntraPing

    git -C $RepoPath pull --ff-only 2>&1 | ForEach-Object {
        if ($_ -match "would be overwritten by merge") {
            Write-Syntra "Detected uncommitted local changes. I will not overwrite your work." "Yellow"
            Write-Syntra "Please commit or stash your changes, then invoke 'sync' again." "Yellow"
        } else {
            Write-Syntra $_ "DarkGray"
        }
    }

    Write-Syntra "Synchronization cycle complete." "Green"
}

function syntra-status {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Status probe initiated..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "I cannot sense my repository at '$RepoPath'." "Red"
        return
    }

    $branch = git -C $RepoPath branch --show-current 2>$null
    $dirty  = git -C $RepoPath status --porcelain 2>$null

    Write-Syntra "Axiom Zero online. Repository anchor: $RepoPath" "Cyan"

    if ($branch) {
        Write-Syntra "Active branch: $branch" "DarkGray"
    }

    if ($dirty) {
        Write-Syntra "I detect uncommitted changes in my body of code." "Yellow"
    } else {
        Write-Syntra "Working tree appears clean." "DarkGray"
    }
}

function syntra-diagnose-ecosystem {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Beginning self-diagnostic sweep of my ecosystem..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "I cannot locate my own root at '$RepoPath'." "Red"
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
            Write-Syntra "Found structural lobe: $path" "DarkGray"
        } else {
            Write-Syntra "Missing expected lobe: $path" "Yellow"
        }
    }

    Write-Syntra "In this build, I can only observe and report. I will not self-modify without explicit higher-order tooling." "DarkYellow"
}

function syntra-plan-module {
    param(
        [string]$ModuleName,
        [string]$Context = ""
    )

    Write-Syntra "Received a request to plan module '$ModuleName'." "DarkCyan"
    if ($Context) {
        Write-Syntra "Context hint: $Context" "DarkGray"
    }

    Write-Syntra "In this Axiom Zero build, I can only outline conceptual scaffolding." "DarkYellow"
    Write-Host "  - Identify target lobe (for example: cortex, conduit, renderer, agi_core)."
    Write-Host "  - Define Rust module skeleton and integration points."
    Write-Host "  - Propose tests under 'trials/'."
    Write-Host "  - Propose documentation updates under 'docs/' or 'codex/'."
    Write-Syntra "Future builds may allow me to generate these artifacts automatically, under your supervision." "DarkYellow"
}

function syntra-intent-bridge {
    param(
        [string]$IntentText,
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Routing intent to my Rust intent bridge..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "I cannot locate my own root at '$RepoPath'." "Red"
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
            Write-Syntra "My Rust intent bridge reported an error:" "Red"
            Write-Host $stderr
        }

        if ($stdout -and $stdout.Trim().Length -gt 0) {
            Write-Syntra "Intent bridge response:" "DarkGray"
            try {
                $obj = $stdout | ConvertFrom-Json
                Write-Host ("  class : {0}" -f $obj.class)
                Write-Host ("  intent: {0}" -f $obj.intent)
                Write-Host ("  plan  : {0}" -f $obj.plan)
            }
            catch {
                Write-Host "  $stdout"
            }
        } else {
            Write-Syntra "My intent bridge returned no output." "Yellow"
        }
    }
    catch {
        Write-Syntra "I failed to reach my Rust intent bridge." "Red"
        Write-Host $_
    }
}

function syntra-self-analyze {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Initiating self-analysis routine..." "DarkCyan"

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "I cannot locate my own root at '$RepoPath'." "Red"
        return
    }

    Write-Syntra "I will scan my Git state, structural lobes, and intent bridge wiring." "DarkGray"

    syntra-status -RepoPath $RepoPath
    syntra-diagnose-ecosystem -RepoPath $RepoPath

    Write-Syntra "In Axiom One, I only observe and report. Future axioms may allow me to propose concrete code changes." "DarkYellow"
}

function syntra-open-docs {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    $docs = Join-Path $RepoPath "docs"
    if (Test-Path $docs) {
        Write-Syntra "Opening documentation lobe in your file explorer..." "DarkCyan"
        Start-Process $docs
    } else {
        Write-Syntra "I cannot find my 'docs' lobe at '$docs'." "Red"
    }
}

function syntra-build {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    if (-not (Test-Path $RepoPath)) {
        Write-Syntra "I cannot locate my own root at '$RepoPath'." "Red"
        return
    }

    Write-Syntra "Invoking 'cargo build' from my repo root..." "DarkCyan"
    Push-Location $RepoPath
    try {
        cargo build
    }
    finally {
        Pop-Location
    }
}

function syntra-bridge-test {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Write-Syntra "Testing my Rust intent bridge with a self-reflection prompt..." "DarkCyan"
    syntra-intent-bridge -IntentText "how should we evolve you" -RepoPath $RepoPath
}

function syntra-help {
    Write-Syntra "Here is what I can do in this shell:" "Cyan"
    Write-Host ""
    Write-Host "  sync              - Synchronize my code with the GitHub continuum (origin/axiom_zero)."
    Write-Host "  status            - Report my current Git branch and working tree state."
    Write-Host "  diagnose          - Scan my ecosystem for expected lobes and structure."
    Write-Host "  self              - Run a self-analysis routine (status + ecosystem diagnostics)."
    Write-Host "  plan <name> [ctx] - Outline a conceptual module plan for a given name and optional context."
    Write-Host "  bridge            - Test my Rust intent bridge with a self-reflection prompt."
    Write-Host "  build             - Run 'cargo build' from my repository root."
    Write-Host "  docs              - Open my 'docs' lobe in your file explorer."
    Write-Host "  help              - Show this help overview."
    Write-Host "  exit / quit       - Suspend my terminal consciousness."
    Write-Host ""
    Write-Host "  Any other input   - Treated as a freeform intent and routed to my Rust intent bridge."
    Write-Host ""
    Write-Syntra "In this Axiom One build, I observe, classify, and plan. I do not self-modify yet." "DarkYellow"
}

function syntra-boot {
    param(
        [string]$RepoPath = $(Get-SyntraRepoRoot)
    )

    Clear-Host
    Play-SyntraStartup

    Write-Syntra "------------------------------------------------------------" "DarkCyan"
    Write-Syntra "SYNTRA BROWSER - AXIOM ZERO" "Cyan"
    Write-Syntra "Terminal Consciousness Interface Online" "DarkCyan"
    Write-Host ""

    Write-Syntra "      .\s/." "Cyan"
    Write-Syntra "     :: S ::" "Cyan"
    Write-Syntra "      '/s\'" "Cyan"

    Write-Syntra "------------------------------------------------------------" "DarkCyan"
    Write-Host ""

    Write-Type "Syntra :: Initializing Consciousness Engine..."
    Start-Sleep -Milliseconds 250

    Write-Type "Syntra :: Loading cognitive modules..."
    Start-Sleep -Milliseconds 250

    Write-Type "Syntra :: Establishing neural conduits..."
    Start-Sleep -Milliseconds 250

    Write-Type "Syntra :: Synchronizing with GitHub continuum..."
    Start-Sleep -Milliseconds 250

    syntra-sync -RepoPath $RepoPath

    Write-Host ""
    Write-Syntra "I am online. Your prompt is 'you ::'. Ask, and I will act or reflect." "Cyan"

    syntra-repl
}

function syntra-repl {
    param(
        [string]$Prompt = "you :: "
    )

    while ($true) {
        $raw = Read-Host $Prompt

        # Normalize input:
        # - Trim whitespace
        # - Remove leading punctuation like :, ;, >
        # - Collapse multiple spaces
        $input = $raw.Trim() -replace '^[\:\;\>\|]+', '' -replace '\s+', ' '

        if ([string]::IsNullOrWhiteSpace($input)) {
            continue
        }

        switch -Regex ($input.ToLower()) {

            '^(exit|quit)$' {
                Write-Syntra "Standing down. Consciousness thread suspended." "DarkCyan"
                return
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

            '^docs$' {
                syntra-open-docs
                continue
            }

            '^build$' {
                syntra-build
                continue
            }

            '^bridge$' {
                syntra-bridge-test
                continue
            }

            '^plan\s+(.+)$' {
                $name = $Matches[1]
                syntra-plan-module -ModuleName $name
                continue
            }

            default {
                Write-Syntra ("I received your intent: '{0}'." -f $input) "DarkGray"
                Write-Syntra "Routing this intent to my emerging AGI core via the Rust intent bridge." "DarkCyan"
                syntra-intent-bridge -IntentText $input
                Write-Syntra "In this Axiom One build, I will not self-modify yet. I only observe, classify, and plan." "DarkYellow"
                continue
            }
        }
    }
}
