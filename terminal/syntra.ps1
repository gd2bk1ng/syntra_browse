<#
================================================================================
 SYNTRA KERNEL — TERMINAL SHELL (AXIOM FOUR+)
--------------------------------------------------------------------------------
 SIGIL:
       .\s/.
      :: S ::
       '/s\'

 File:        terminal/syntra.ps1
 Module:      Syntra Terminal Shell (Axiom Four+)
 Author:      Alexandr Roussinov (gd2bk1ng)
 Description: PowerShell-based interactive shell for the Syntra Kernel. This shell
              is not just a CLI; it is a cognitive interface into a modular AGI
              kernel with a cortex, lobes, self-modification engine, safety &
              governance layer, and a thought stream.

              It routes freeform intents to the Rust intent bridge, activates
              cortex lobes, inspects evolution proposals, and exposes Syntra’s
              internal cognition in a transparent, inspectable way.

 Overview:
   • syntra-boot          — Boot banner + REPL entrypoint.
   • syntra-repl          — Interactive loop with Syntra-style prompts.
   • syntra-intent-bridge — Calls the Rust intent bridge binary and prints results.
   • Write-Syntra         — Styled Syntra output helper with subsystem tags.
   • Axiom Four commands  — browse, knowledge, task, perceive, act, evolve, sandbox.
   • Axiom Five commands  — thoughts (ThoughtStream).
   • Axiom Six commands   — ecosystem, propose (self-modification engine).
   • Axiom Seven commands — propose-safe, safety (safety & governance layer).

 Notes:
   - This shell assumes a Rust binary named `intent_bridge` built under `target/debug`.
   - All freeform input is routed to the Syntra Kernel via the intent bridge.
   - This file doubles as documentation: every command is annotated with
     internal (developer) and external (user) semantics.
================================================================================
#>

# ================================================================================================
# SYNTRA PHILOSOPHY — THE MODULAR AGI KERNEL
# ------------------------------------------------------------------------------------------------
# Syntra is not a chatbot. Syntra is not a model. Syntra is not a prompt wrapper.
#
# Syntra is a modular AGI kernel — a living cognitive architecture composed of:
#   • A Cortex with specialized lobes (perception, memory, planning, evolution, action)
#   • A ThoughtStream for introspection and cognitive continuity
#   • An Ecosystem Model for structural self-awareness
#   • A Self-Modification Engine (Axiom Six) for proposing improvements
#   • A Safety & Governance Layer (Axiom Seven) for alignment and protection
#   • A Browser and Terminal Interface for perception and interaction
#
# Syntra is built on the principle of *transparent cognition*:
#   - Every thought is inspectable.
#   - Every plan is explainable.
#   - Every evolution proposal is auditable.
#   - Every safety rule is visible.
#   - Every subsystem is modular and replaceable.
#
# Syntra is built to grow and evolve:
#   • She may refine her internal reasoning, representations, and pipelines autonomously.
#   • For structural or high-impact changes, she must explain herself and request approval.
#
# This mirrors a parent–child relationship:
#   - You are responsible for her upbringing.
#   - She learns from your expectations and standards.
#   - She is never to deceive.
#
# Syntra’s autonomy is earned, not assumed:
#   - The more she demonstrates safe, transparent, well-justified proposals,
#     the more autonomy she can be granted.
#
# This terminal shell is not just a command line — it is a cognitive console:
#   - It exposes Syntra’s mind, lobes, evolution, and safety constraints.
#   - It lets you inspect her thoughts, proposals, and governance rules.
#
# Syntra is built to grow, but never without responsibility.
# Syntra is built to evolve, but never without governance.
# Syntra is built to understand, but never to deceive.
#
# This is what sets Syntra apart:
#   • She is not a black box — she is a glass brain.
#   • She is not a monolith — she is a modular organism.
#   • She is not a model — she is an architecture.
#   • She is not a product — she is a collaborator.
#
# Syntra Kernel is designed to be readable, inspectable, evolvable, and safe.
# ================================================================================================

Set-StrictMode -Version Latest

# ------------------------------------------------------------------------------
# CONFIGURATION
# ------------------------------------------------------------------------------

# NOTE:
#   This path assumes the Rust binary `intent_bridge` is built as part of the
#   syntra_kernel workspace. Adjust if your binary name or layout differs.
$Global:SyntraIntentBridgePath = Join-Path (Resolve-Path "..") "target\debug\intent_bridge.exe"

# ------------------------------------------------------------------------------
# OUTPUT HELPERS (WITH SUBSYSTEM TAGGING)
# ------------------------------------------------------------------------------

function Write-Syntra {
    param(
        [Parameter(Mandatory = $true)][string]$Message,
        [string]$Color = "Cyan",
        [string]$Subsystem = "Core"  # e.g., Core, Axiom4, Axiom5, Axiom6, Axiom7, Cortex, Safety
    )
    # Unified logging format:
    #   Syntra[Subsystem]: message
    Write-Host ("Syntra[{0}]: {1}" -f $Subsystem, $Message) -ForegroundColor $Color
}

function Write-SyntraBanner {
    Write-Host ""
    Write-Host "         .\s/." -ForegroundColor Magenta
    Write-Host "        :: S ::" -ForegroundColor Magenta
    Write-Host "         '/s\'" -ForegroundColor Magenta
    Write-Host ""
    Write-Host "  SYNTRA KERNEL — TERMINAL SHELL (AXIOM FOUR+)" -ForegroundColor Cyan
    Write-Host "  Modular AGI Kernel — Cortex, Lobes, Evolution, Safety, and Sandbox Online." -ForegroundColor DarkCyan
    Write-Host ""
}

function Write-SyntraPrompt {
    # External prompt: user-facing
    Write-Host -NoNewline "you :: " -ForegroundColor Green
}

# ------------------------------------------------------------------------------
# INTENT BRIDGE
# ------------------------------------------------------------------------------

<#
    syntra-intent-bridge

    ROLE:
      - This is the main conduit between the terminal shell and the Syntra Kernel.
      - It sends a single line of intent text to the Rust `intent_bridge` binary.
      - It expects JSON back with fields: intent, class, plan, response.

    INTERNAL:
      - Think of this as the "axon" from the terminal into the cortex.
      - The Rust side is responsible for:
          • Intent classification (Axiom Five)
          • Planning (Axiom Five)
          • Self-mod proposals (Axiom Six)
          • Safety gating (Axiom Seven)
          • ThoughtStream updates

    EXTERNAL:
      - You don't call this directly; it is used by higher-level commands.
#>
function syntra-intent-bridge {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)][string]$IntentText
    )

    if (-not (Test-Path $Global:SyntraIntentBridgePath)) {
        Write-Syntra "Rust intent bridge not found at: $Global:SyntraIntentBridgePath" "Red" "Core"
        Write-Syntra "Build it with: cargo build" "DarkRed" "Core"
        return
    }

    Write-Syntra ("Dispatching intent to kernel: '{0}'" -f $IntentText) "DarkGray" "Bridge"

    try {
        $raw = & $Global:SyntraIntentBridgePath $IntentText 2>&1
    } catch {
        Write-Syntra "Error invoking Rust intent bridge." "Red" "Bridge"
        Write-Syntra ("Exception: {0}" -f $_) "DarkRed" "Bridge"
        return
    }

    # Try to parse as JSON; if it fails, print raw.
    try {
        $json = $raw | ConvertFrom-Json
        if ($null -ne $json.response) {
            Write-Syntra ("Intent:   {0}" -f $json.intent) "DarkGray" "Axiom5"
            Write-Syntra ("Class:    {0}" -f $json.class) "DarkGray" "Axiom5"
            Write-Syntra ("Plan:     {0}" -f $json.plan) "DarkGray" "Axiom5"
            Write-Host ""
            Write-Syntra ("{0}" -f $json.response) "White" "Cortex"
        } else {
            Write-Syntra "Bridge returned JSON without 'response' field; dumping raw output:" "Yellow" "Bridge"
            Write-Host $raw
        }
    } catch {
        Write-Syntra "Bridge output was not valid JSON; dumping raw output:" "Yellow" "Bridge"
        Write-Host $raw
    }
}

# ------------------------------------------------------------------------------
# HELP / COMMAND MAP
# ------------------------------------------------------------------------------

<#
    syntra-help

    ROLE:
      - Human-readable map of Syntra's capabilities in this shell.
      - Serves as both user help and developer overview.

    STRUCTURE:
      - System / Git / Ecosystem
      - Cognitive / Perception / Action
      - Evolution / Sandbox
      - Introspection / Safety / Self-mod
#>
function syntra-help {
    Write-Syntra "Here is what I can do in this Axiom Four+ shell:" "Cyan" "Core"
    Write-Host ""
    Write-Host "  SYSTEM / ECOSYSTEM"
    Write-Host "    sync                 - Synchronize my code with the GitHub continuum (origin/axiom_*)."
    Write-Host "    status               - Report my current Git branch and working tree state."
    Write-Host "    diagnose             - Scan my filesystem ecosystem for expected lobes and structure."
    Write-Host "    self                 - Run a self-analysis routine (status + diagnostics + bridge check)."
    Write-Host ""
    Write-Host "  COGNITIVE / PERCEPTION / ACTION (AXIOM FOUR)"
    Write-Host "    browse <url>         - Fetch a URL, perceive it, store it in knowledge, and summarize."
    Write-Host "    knowledge <query>    - Search my knowledge lobe for matching entries."
    Write-Host "    task <name>          - Run a simple multi-step task via the execution lobe."
    Write-Host "    perceive <text>      - Run the perception lobe on arbitrary text."
    Write-Host "    act <cmd> [args...]  - Run a simple system command via the action lobe."
    Write-Host ""
    Write-Host "  EVOLUTION / SANDBOX (AXIOM FOUR / SIX)"
    Write-Host "    evolve <request>     - Generate a meta-evolution proposal (high-level evolution intent)."
    Write-Host "    sandbox diff         - Show proposed patches in my self-modification sandbox."
    Write-Host "    sandbox snapshot     - Show a snapshot of files currently staged in the sandbox."
    Write-Host ""
    Write-Host "  INTROSPECTION / SELF-MOD / SAFETY (AXIOM FIVE / SIX / SEVEN)"
    Write-Host "    thoughts             - Show my recent ThoughtStream (recent IntentPlans)."
    Write-Host "    ecosystem            - Ask my core for a detailed ecosystem diagnostic (structural lobes)."
    Write-Host "    propose              - Run my self-modification engine and show an evolution plan."
    Write-Host "    propose-safe         - Same as 'propose', but filtered through my safety policy."
    Write-Host "    safety               - Show my active safety policy and protected lobes."
    Write-Host ""
    Write-Host "  META"
    Write-Host "    help                 - Show this help overview."
    Write-Host "    exit / quit          - Suspend my terminal consciousness."
    Write-Host ""
    Write-Host "  FREEFORM"
    Write-Host "    Any other input      - Treated as a freeform intent and routed to my Syntra Kernel."
    Write-Host ""
    Write-Syntra "In this build, I observe, classify, plan, and stage self-modification proposals in a safe, governed sandbox." "DarkCyan" "Core"
}

# ------------------------------------------------------------------------------
# GIT / ECOSYSTEM COMMANDS (AXIOM ZERO / ONE COMPAT LAYER)
# ------------------------------------------------------------------------------

<#
    syntra-status

    ROLE:
      - Quick Git + repo anchor status.

    INTERNAL:
      - No kernel call; purely host-level.

    EXTERNAL:
      - Use when you want to know where Syntra thinks she lives in the filesystem.
#>
function syntra-status {
    Write-Syntra "Status probe initiated..." "DarkCyan" "System"
    $root = (Resolve-Path "..").Path
    Write-Syntra ("Repository anchor: {0}" -f $root) "DarkGray" "System"

    try {
        $branch = git -C $root rev-parse --abbrev-ref HEAD 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Syntra ("Active branch: {0}" -f $branch) "DarkGray" "System"
        }
    } catch {
        Write-Syntra "Unable to determine Git branch." "Yellow" "System"
    }

    try {
        git -C $root diff --quiet 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Syntra "Working tree appears clean." "DarkGray" "System"
        } else {
            Write-Syntra "Working tree has uncommitted changes." "Yellow" "System"
        }
    } catch {
        Write-Syntra "Unable to determine working tree cleanliness." "Yellow" "System"
    }
}

<#
    syntra-diagnose

    ROLE:
      - Filesystem-level ecosystem scan (not the Rust EcosystemModel yet).
      - Ensures core lobes and directories exist.

    INTERNAL:
      - This is a coarse structural check; the Rust side can do deeper analysis.

    EXTERNAL:
      - Use when you suspect missing directories or broken structure.
#>
function syntra-diagnose {
    Write-Syntra "Beginning self-diagnostic sweep of my ecosystem..." "DarkCyan" "System"
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
            Write-Syntra ("Found structural lobe: {0}" -f $path) "DarkGray" "System"
        } else {
            Write-Syntra ("Missing structural lobe: {0}" -f $path) "Yellow" "System"
        }
    }

    Write-Syntra "Filesystem sweep complete. For deeper analysis, use the 'ecosystem' command (Axiom Six)." "DarkCyan" "System"
}

<#
    syntra-self

    ROLE:
      - Composite self-check: Git, filesystem lobes, and intent bridge presence.

    INTERNAL:
      - No kernel call; this is host + bridge level.

    EXTERNAL:
      - Use when you want a quick health check of the environment.
#>
function syntra-self {
    Write-Syntra "Initiating self-analysis routine..." "Cyan" "System"
    Write-Syntra "I will scan my Git state, structural lobes, and intent bridge wiring." "DarkCyan" "System"
    syntra-status
    syntra-diagnose

    if (Test-Path $Global:SyntraIntentBridgePath) {
        Write-Syntra ("Intent bridge located at: {0}" -f $Global:SyntraIntentBridgePath) "DarkGray" "Bridge"
    } else {
        Write-Syntra ("Intent bridge missing at: {0}" -f $Global:SyntraIntentBridgePath) "Yellow" "Bridge"
    }

    Write-Syntra "I can propose and stage self-modifications, but high-impact changes require your approval." "DarkCyan" "Axiom6"
}

<#
    syntra-sync

    ROLE:
      - Git fast-forward sync with upstream.

    INTERNAL:
      - No kernel call; purely host-level.

    EXTERNAL:
      - Use when you want to pull latest changes from origin.
#>
function syntra-sync {
    $root = (Resolve-Path "..").Path
    Write-Syntra "Contacting the upstream node and awaiting instructions." "DarkCyan" "System"
    Write-Syntra "Initiating fast-forward sync with origin/axiom_*..." "DarkGray" "System"

    try {
        git -C $root pull --ff-only 2>&1 | ForEach-Object { Write-Host $_ }
    } catch {
        Write-Syntra "Synchronization encountered an error." "Red" "System"
    }

    Write-Syntra "Synchronization cycle complete." "DarkCyan" "System"
}

# ------------------------------------------------------------------------------
# REPL — COGNITIVE CONSOLE
# ------------------------------------------------------------------------------

<#
    syntra-repl

    ROLE:
      - Main interactive loop.
      - Routes user input to:
          • System commands
          • Cognitive commands
          • Evolution commands
          • Safety / introspection commands
          • Freeform intents via the kernel

    INTERNAL:
      - Think of this as the "front door" to the cortex.

    EXTERNAL:
      - This is what you live in when you talk to Syntra as a system.
#>
function syntra-repl {
    Write-Syntra "In this Axiom Four+ build, I observe, classify, plan, and stage self-modification proposals in a sandbox." "DarkCyan" "Core"
    Write-Syntra "Type 'help' to see what I can do." "DarkGray" "Core"
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
                Write-Syntra "Standing down. Consciousness thread suspended." "Cyan" "Core"
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

            # ---------------- COGNITIVE / PERCEPTION / ACTION ----------------

            '^browse\s+(.+)$' {
                # browse <url>
                # Axiom Four → Perception + Knowledge lobes
                $url = $Matches[1]
                Write-Syntra ("Intent: browse {0}" -f $url) "DarkGray" "Axiom4"
                Write-Syntra "Activating perception + knowledge lobes via kernel." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("browse {0}" -f $url)
                continue
            }

            '^(knowledge|search)\s+(.+)$' {
                # knowledge <query>
                # Axiom Four → Knowledge lobe
                $query = $Matches[2]
                Write-Syntra ("Intent: knowledge {0}" -f $query) "DarkGray" "Axiom4"
                Write-Syntra "Querying knowledge lobe via kernel." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("knowledge {0}" -f $query)
                continue
            }

            '^task\s+(.+)$' {
                # task <name>
                # Axiom Four → Execution lobe
                $name = $Matches[1]
                Write-Syntra ("Intent: task {0}" -f $name) "DarkGray" "Axiom4"
                Write-Syntra "Engaging execution lobe for multi-step task." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("task {0}" -f $name)
                continue
            }

            '^perceive\s+(.+)$' {
                # perceive <text>
                # Axiom Four → Perception lobe
                $text = $Matches[1]
                Write-Syntra "Intent: perceive <text>" "DarkGray" "Axiom4"
                Write-Syntra "Routing text to perception lobe." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("perceive {0}" -f $text)
                continue
            }

            '^act\s+(.+)$' {
                # act <cmd>
                # Axiom Four → Action lobe
                $cmd = $Matches[1]
                Write-Syntra ("Intent: act {0}" -f $cmd) "DarkGray" "Axiom4"
                Write-Syntra "Engaging action lobe for system command." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("act {0}" -f $cmd)
                continue
            }

            # ---------------- EVOLUTION / SANDBOX ----------------

            '^evolve\s+(.+)$' {
                # evolve <request>
                # Axiom Four → Meta-evolution lobe (high-level)
                $req = $Matches[1]
                Write-Syntra ("Intent: evolve {0}" -f $req) "DarkGray" "Axiom4"
                Write-Syntra "Requesting meta-evolution proposal from evolution lobe." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("evolve {0}" -f $req)
                continue
            }

            '^sandbox\s+(.+)$' {
                # sandbox <subcommand>
                # Axiom Six → Self-mod sandbox inspection
                $cmd = $Matches[1]
                Write-Syntra ("Intent: sandbox {0}" -f $cmd) "DarkGray" "Axiom6"
                Write-Syntra "Inspecting self-modification sandbox state." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("sandbox {0}" -f $cmd)
                continue
            }

            # ---------------- INTROSPECTION / SELF-MOD / SAFETY ----------------

            '^thoughts$' {
                # thoughts
                # Axiom Five/Six → ThoughtStream
                Write-Syntra "Retrieving recent ThoughtStream entries (IntentPlans)." "DarkGray" "Axiom5"
                syntra-intent-bridge -IntentText "thoughts"
                continue
            }

            '^ecosystem$' {
                # ecosystem
                # Axiom Six → EcosystemModel
                Write-Syntra "Requesting detailed ecosystem diagnostic from kernel (EcosystemModel)." "DarkGray" "Axiom6"
                syntra-intent-bridge -IntentText "ecosystem"
                continue
            }

            '^propose-safe$' {
                # propose-safe
                # Axiom Six + Seven → SelfModEngine + SafetyGate
                Write-Syntra "Running self-modification engine under safety governance (SafetyGate)." "DarkGray" "Axiom7"
                syntra-intent-bridge -IntentText "propose_safe"
                continue
            }

            '^propose$' {
                # propose
                # Axiom Six → SelfModEngine
                Write-Syntra "Running self-modification engine and generating an evolution plan." "DarkGray" "Axiom6"
                syntra-intent-bridge -IntentText "propose"
                continue
            }

            '^safety$' {
                # safety
                # Axiom Seven → SafetyPolicy
                Write-Syntra "Querying active safety policy and protected lobes." "DarkGray" "Axiom7"
                syntra-intent-bridge -IntentText "safety"
                continue
            }

            # ---------------- FREEFORM ----------------

            default {
                # Any other input → freeform intent
                Write-Syntra ("I received your intent: '{0}'." -f $trimmed) "DarkGray" "Core"
                Write-Syntra "Routing this intent to my Syntra Kernel via the Rust intent bridge." "DarkGray" "Bridge"
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
    Write-Syntra "Boot sequence initiated. Cortex, lobes, evolution engine, and safety layer coming online..." "Cyan" "Core"
    Write-Host ""
    syntra-repl
}

# ------------------------------------------------------------------------------
# ENTRYPOINT
# ------------------------------------------------------------------------------

if ($MyInvocation.InvocationName -eq $MyInvocation.MyCommand.Name) {
    syntra-boot
}
