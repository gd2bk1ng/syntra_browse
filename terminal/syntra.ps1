<#
================================================================================
 SYNTRA BROWSER — AXIOM FOUR
--------------------------------------------------------------------------------
 SIGIL:
       .\s/.
      :: S ::
       '/s\'

 File:        terminal/syntra.ps1
 Module:      Syntra Terminal Shell (Axiom Four)
 Author:      Alexandr Roussinov (gd2bk1ng)
 Description: PowerShell-based interactive shell for the Syntra Browser. This shell
              boots Syntra’s Axiom Four cognitive stack, routes freeform intents to
              the Rust intent bridge, and exposes high-level commands for browsing,
              knowledge retrieval, task execution, perception, actions, evolution,
              and sandbox introspection.

 Overview:
   • syntra-boot          — Boot banner + REPL entrypoint.
   • syntra-repl          — Interactive loop with Syntra-style prompts.
   • syntra-intent-bridge — Calls the Rust intent bridge binary and prints results.
   • Write-Syntra         — Styled Syntra output helper.
   • Axiom Four commands  — browse, knowledge, task, perceive, act, evolve, sandbox.

 Notes:
   - This shell assumes a Rust binary named `intent_bridge` built under `target/debug`.
   - All freeform input is routed to the Rust AGI core via the intent bridge.
   - Axiom Four messaging replaces older Axiom Zero / One personality lines.
================================================================================
#>

Set-StrictMode -Version Latest

# ------------------------------------------------------------------------------
# CONFIGURATION
# ------------------------------------------------------------------------------

# Path to the Rust intent bridge binary (adjust if your crate/bin name differs).
$Global:SyntraIntentBridgePath = Join-Path (Resolve-Path "..") "target\debug\intent_bridge.exe"

# ------------------------------------------------------------------------------
# OUTPUT HELPERS
# ------------------------------------------------------------------------------

function Write-Syntra {
    param(
        [Parameter(Mandatory = $true)][string]$Message,
        [string]$Color = "Cyan"
    )
    Write-Host "Syntra: $Message" -ForegroundColor $Color
}

function Write-SyntraBanner {
    Write-Host ""
    Write-Host "         .\s/." -ForegroundColor Magenta
    Write-Host "        :: S ::" -ForegroundColor Magenta
    Write-Host "         '/s\'" -ForegroundColor Magenta
    Write-Host ""
    Write-Host "  SYNTRA BROWSER — AXIOM FOUR" -ForegroundColor Cyan
    Write-Host "  Native AGI Browser — Cortex, Lobes, and Sandbox Online." -ForegroundColor DarkCyan
    Write-Host ""
}

function Write-SyntraPrompt {
    Write-Host -NoNewline "you :: " -ForegroundColor Green
}

# ------------------------------------------------------------------------------
# INTENT BRIDGE
# ------------------------------------------------------------------------------

function syntra-intent-bridge {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)][string]$IntentText
    )

    if (-not (Test-Path $Global:SyntraIntentBridgePath)) {
        Write-Syntra "Rust intent bridge not found at: $Global:SyntraIntentBridgePath" "Red"
        Write-Syntra "Build it with: cargo build" "DarkRed"
        return
    }

    try {
        $raw = & $Global:SyntraIntentBridgePath $IntentText 2>&1
    } catch {
        Write-Syntra "Error invoking Rust intent bridge: $_" "Red"
        return
    }

    # Try to parse as JSON; if it fails, print raw.
    try {
        $json = $raw | ConvertFrom-Json
        if ($null -ne $json.response) {
            Write-Syntra ("Intent:   {0}" -f $json.intent) "DarkGray"
            Write-Syntra ("Class:    {0}" -f $json.class) "DarkGray"
            Write-Syntra ("Plan:     {0}" -f $json.plan) "DarkGray"
            Write-Host ""
            Write-Syntra ("{0}" -f $json.response) "White"
        } else {
            Write-Syntra "Bridge returned JSON without 'response' field; dumping raw output:" "Yellow"
            Write-Host $raw
        }
    } catch {
        Write-Syntra "Bridge output was not valid JSON; dumping raw output:" "Yellow"
        Write-Host $raw
    }
}

# ------------------------------------------------------------------------------
# HELP
# ------------------------------------------------------------------------------

function syntra-help {
    Write-Syntra "Here is what I can do in this Axiom Four shell:" "Cyan"
    Write-Host ""
    Write-Host "  sync                 - Synchronize my code with the GitHub continuum (origin/axiom_*)." 
    Write-Host "  status               - Report my current Git branch and working tree state."
    Write-Host "  diagnose             - Scan my ecosystem for expected lobes and structure."
    Write-Host "  self                 - Run a self-analysis routine (status + diagnostics)."
    Write-Host ""
    Write-Host "  browse <url>         - Fetch a URL, perceive it, store it in knowledge, and summarize."
    Write-Host "  knowledge <query>    - Search my knowledge lobe for matching entries."
    Write-Host "  task <name>          - Run a simple multi-step task via the execution lobe."
    Write-Host "  perceive <text>      - Run the perception lobe on arbitrary text."
    Write-Host "  act <cmd> [args...]  - Run a simple system command via the action lobe."
    Write-Host ""
    Write-Host "  evolve <request>     - Generate a meta-evolution proposal (Axiom Four)."
    Write-Host "  sandbox diff         - Show proposed patches in my self-modification sandbox."
    Write-Host "  sandbox snapshot     - Show a snapshot of files currently staged in the sandbox."
    Write-Host ""
    Write-Host "  help                 - Show this help overview."
    Write-Host "  exit / quit          - Suspend my terminal consciousness."
    Write-Host ""
    Write-Host "  Any other input      - Treated as a freeform intent and routed to my Rust intent bridge."
    Write-Host ""
    Write-Syntra "In this Axiom Four build, I observe, classify, plan, and stage self-modification proposals in a safe sandbox." "DarkCyan"
}

# ------------------------------------------------------------------------------
# GIT / ECOSYSTEM COMMANDS (AXIOM ZERO / ONE COMPAT LAYER)
# ------------------------------------------------------------------------------

function syntra-status {
    Write-Syntra "Status probe initiated..." "DarkCyan"
    $root = (Resolve-Path "..").Path
    Write-Syntra ("Repository anchor: {0}" -f $root) "DarkGray"

    try {
        $branch = git -C $root rev-parse --abbrev-ref HEAD 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Syntra ("Active branch: {0}" -f $branch) "DarkGray"
        }
    } catch {}

    try {
        git -C $root diff --quiet 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Syntra "Working tree appears clean." "DarkGray"
        } else {
            Write-Syntra "Working tree has uncommitted changes." "Yellow"
        }
    } catch {}
}

function syntra-diagnose {
    Write-Syntra "Beginning self-diagnostic sweep of my ecosystem..." "DarkCyan"
    $root = (Resolve-Path "..").Path

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
        $full = Join-Path $root $path
        if (Test-Path $full) {
            Write-Syntra ("Found structural lobe: {0}" -f $path) "DarkGray"
        } else {
            Write-Syntra ("Missing structural lobe: {0}" -f $path) "Yellow"
        }
    }

    Write-Syntra "In this build, I observe, report, and now stage evolution proposals via my sandbox." "DarkCyan"
}

function syntra-self {
    Write-Syntra "Initiating self-analysis routine..." "Cyan"
    Write-Syntra "I will scan my Git state, structural lobes, and intent bridge wiring." "DarkCyan"
    syntra-status
    syntra-diagnose

    if (Test-Path $Global:SyntraIntentBridgePath) {
        Write-Syntra ("Intent bridge located at: {0}" -f $Global:SyntraIntentBridgePath) "DarkGray"
    } else {
        Write-Syntra ("Intent bridge missing at: {0}" -f $Global:SyntraIntentBridgePath) "Yellow"
    }

    Write-Syntra "In Axiom Four, I can propose and stage self-modifications, but I will not apply them without your approval." "DarkCyan"
}

function syntra-sync {
    $root = (Resolve-Path "..").Path
    Write-Syntra "Contacting the upstream node and awaiting instructions." "DarkCyan"
    Write-Syntra "Initiating fast-forward sync with origin/axiom_*..." "DarkGray"

    try {
        git -C $root pull --ff-only 2>&1 | ForEach-Object { Write-Host $_ }
    } catch {
        Write-Syntra "Synchronization encountered an error." "Red"
    }

    Write-Syntra "Synchronization cycle complete." "DarkCyan"
}

# ------------------------------------------------------------------------------
# REPL
# ------------------------------------------------------------------------------

function syntra-repl {
    Write-Syntra "In this Axiom Four build, I observe, classify, plan, and stage self-modification proposals in a sandbox." "DarkCyan"
    Write-Syntra "Type 'help' to see what I can do." "DarkGray"
    Write-Host ""

    while ($true) {
        Write-SyntraPrompt
        $line = Read-Host

        if ([string]::IsNullOrWhiteSpace($line)) {
            continue
        }

        $trimmed = $line.Trim()

        switch -Regex ($trimmed) {
            '^(exit|quit)$' {
                Write-Syntra "Standing down. Consciousness thread suspended." "Cyan"
                break
            }

            '^help$' {
                syntra-help
                continue
            }

            '^status$' {
                syntra-status
                continue
            }

            '^diagnose$' {
                syntra-diagnose
                continue
            }

            '^self$' {
                syntra-self
                continue
            }

            '^sync$' {
                syntra-sync
                continue
            }

            '^browse\s+(.+)$' {
                $url = $Matches[1]
                Write-Syntra ("I received your intent: 'browse {0}'." -f $url) "DarkGray"
                syntra-intent-bridge -IntentText ("browse {0}" -f $url)
                continue
            }

            '^(knowledge|search)\s+(.+)$' {
                $query = $Matches[2]
                Write-Syntra ("Searching knowledge for '{0}'." -f $query) "DarkGray"
                syntra-intent-bridge -IntentText ("knowledge {0}" -f $query)
                continue
            }

            '^task\s+(.+)$' {
                $name = $Matches[1]
                Write-Syntra ("Running task '{0}'." -f $name) "DarkGray"
                syntra-intent-bridge -IntentText ("task {0}" -f $name)
                continue
            }

            '^perceive\s+(.+)$' {
                $text = $Matches[1]
                Write-Syntra "Perceiving provided text..." "DarkGray"
                syntra-intent-bridge -IntentText ("perceive {0}" -f $text)
                continue
            }

            '^act\s+(.+)$' {
                $cmd = $Matches[1]
                Write-Syntra ("Executing action: {0}" -f $cmd) "DarkGray"
                syntra-intent-bridge -IntentText ("act {0}" -f $cmd)
                continue
            }

            '^evolve\s+(.+)$' {
                $req = $Matches[1]
                Write-Syntra ("Generating evolution proposal for: {0}" -f $req) "DarkGray"
                syntra-intent-bridge -IntentText ("evolve {0}" -f $req)
                continue
            }

            '^sandbox\s+(.+)$' {
                $cmd = $Matches[1]
                Write-Syntra ("Inspecting sandbox: {0}" -f $cmd) "DarkGray"
                syntra-intent-bridge -IntentText ("sandbox {0}" -f $cmd)
                continue
            }

            default {
                Write-Syntra ("I received your intent: '{0}'." -f $trimmed) "DarkGray"
                Write-Syntra "Routing this intent to my Axiom Four AGI core via the Rust intent bridge." "DarkGray"
                syntra-intent-bridge -IntentText $trimmed
                continue
            }
        }
    }
}

# ------------------------------------------------------------------------------
# BOOT
# ------------------------------------------------------------------------------

function syntra-boot {
    Write-SyntraBanner
    Write-Syntra "Boot sequence initiated. Cortex, lobes, and sandbox coming online..." "Cyan"
    Write-Host ""
    syntra-repl
}
