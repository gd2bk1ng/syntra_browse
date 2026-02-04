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
   • Extended commands    — continuity, simulate, cluster, diagnostics.

 Notes:
   - This shell assumes a Rust binary named `intent_bridge` built under `target/debug`
     at the repository root.
   - All freeform input is routed to the Syntra Kernel via the intent bridge.
   - This file doubles as documentation: every command is annotated with
     internal (developer) and external (user) semantics.
================================================================================
#>

# ================================================================================================
# SYNTRA PHILOSOPHY — THE MODULAR AGI KERNEL
# ================================================================================================

Set-StrictMode -Version Latest

# ------------------------------------------------------------------------------
# CONFIGURATION
# ------------------------------------------------------------------------------

# Anchor everything off the script location so it works regardless of CWD.
$Global:SyntraRepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$Global:SyntraIntentBridgePath = Join-Path $Global:SyntraRepoRoot "target\debug\intent_bridge.exe"

# ------------------------------------------------------------------------------
# OUTPUT HELPERS (WITH SUBSYSTEM TAGGING)
# ------------------------------------------------------------------------------

function Write-Syntra {
    param(
        [Parameter(Mandatory = $true)][string]$Message,
        [string]$Color = "Cyan",
        [string]$Subsystem = "Core"  # e.g., Core, Axiom4, Axiom5, Axiom6, Axiom7, Cortex, Safety, Continuity, Simulation, Cluster, Diagnostics
    )
    Write-Host ("Syntra[{0}]: {1}" -f $Subsystem, $Message) -ForegroundColor $Color
}

function Write-SyntraBanner {
    Write-Host ""
    Write-Host "         .\s/." -ForegroundColor Magenta
    Write-Host "        :: S ::" -ForegroundColor Magenta
    Write-Host "         '/s\'" -ForegroundColor Magenta
    Write-Host ""
    Write-Host "  SYNTRA KERNEL — TERMINAL SHELL (AXIOM FOUR+)" -ForegroundColor Cyan
    Write-Host "  Modular AGI Kernel — Cortex, Lobes, Evolution, Safety, Continuity, Simulation, and Cluster Online." -ForegroundColor DarkCyan
    Write-Host ""
}

function Write-SyntraPrompt {
    Write-Host -NoNewline "you :: " -ForegroundColor Green
}

# ------------------------------------------------------------------------------
# INTENT BRIDGE
# ------------------------------------------------------------------------------

<#
    syntra-intent-bridge

    ROLE:
      - Main conduit between the terminal shell and the Syntra Kernel.
      - Sends a single line of intent text to the Rust `intent_bridge` binary.
      - Expects JSON back with fields: intent, class, plan, response.
#>
function syntra-intent-bridge {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)][string]$IntentText
    )

    if (-not (Test-Path $Global:SyntraIntentBridgePath)) {
        Write-Syntra "Rust intent bridge not found at: $Global:SyntraIntentBridgePath" "Red" "Core"
        Write-Syntra "Build it with: cargo build --bin intent_bridge" "DarkRed" "Core"
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

function syntra-help {
    Write-Syntra "Here is what I can do in this Axiom Four+ shell:" "Cyan" "Core"
    Write-Host ""
    Write-Host "  SYSTEM / ECOSYSTEM"
    Write-Host "    sync                     - Synchronize my code with the GitHub continuum (origin/axiom_*)."
    Write-Host "    status                   - Report my current Git branch and working tree state."
    Write-Host "    diagnose                 - Scan my filesystem ecosystem for expected lobes and structure."
    Write-Host "    self                     - Run a self-analysis routine (status + diagnostics + bridge check)."
    Write-Host ""
    Write-Host "  COGNITIVE / PERCEPTION / ACTION (AXIOM FOUR)"
    Write-Host "    browse <url>             - Fetch a URL, perceive it, store it in knowledge, and summarize."
    Write-Host "    knowledge <query>        - Search my knowledge lobe for matching entries."
    Write-Host "    task <name>              - Run a simple multi-step task via the execution lobe."
    Write-Host "    perceive <text>          - Run the perception lobe on arbitrary text."
    Write-Host "    act <cmd> [args...]      - Run a simple system command via the action lobe."
    Write-Host ""
    Write-Host "  CONTINUITY ENGINE (EPISODIC / PERSISTENCE / STITCHING)"
    Write-Host "    continuity episodic log <text>      - Record an episodic event."
    Write-Host "    continuity episodic list            - List known episodes."
    Write-Host "    continuity episodic show <id>       - Show a specific episode."
    Write-Host "    continuity persist snapshot         - Persist current continuity state."
    Write-Host "    continuity stitch                   - Run stitching over recent events."
    Write-Host ""
    Write-Host "  SIMULATION SANDBOX (WORLD / AGENT / DYNAMICS)"
    Write-Host "    simulate world init <profile>       - Initialize a world profile."
    Write-Host "    simulate agent spawn <spec>         - Spawn a simulated agent."
    Write-Host "    simulate step [n]                   - Advance the simulation by n steps (default 1)."
    Write-Host "    simulate state                      - Summarize current simulation state."
    Write-Host ""
    Write-Host "  DISTRIBUTED RUNTIME (NODE / CLUSTER / MESSAGING)"
    Write-Host "    cluster status                      - Show cluster topology and node health."
    Write-Host "    cluster join <addr>                 - Join a remote cluster or node."
    Write-Host "    cluster message <node> <payload>    - Send a message to a node."
    Write-Host ""
    Write-Host "  DIAGNOSTICS EXT (PROFILER / TELEMETRY / METRICS)"
    Write-Host "    diagnostics profiler snapshot       - Capture a profiler snapshot."
    Write-Host "    diagnostics telemetry stream        - Show recent telemetry events."
    Write-Host "    diagnostics metrics summary         - Show key runtime metrics."
    Write-Host ""
    Write-Host "  EVOLUTION / SANDBOX (AXIOM FOUR / SIX)"
    Write-Host "    evolve <request>                    - Generate a meta-evolution proposal (high-level evolution intent)."
    Write-Host "    sandbox diff                        - Show proposed patches in my self-modification sandbox."
    Write-Host "    sandbox snapshot                    - Show a snapshot of files currently staged in the sandbox."
    Write-Host ""
    Write-Host "  INTROSPECTION / SELF-MOD / SAFETY (AXIOM FIVE / SIX / SEVEN)"
    Write-Host "    thoughts                            - Show my recent ThoughtStream (recent IntentPlans)."
    Write-Host "    ecosystem                           - Ask my core for a detailed ecosystem diagnostic (structural lobes)."
    Write-Host "    propose                             - Run my self-modification engine and show an evolution plan."
    Write-Host "    propose-safe                        - Same as 'propose', but filtered through my safety policy."
    Write-Host "    safety                              - Show my active safety policy and protected lobes."
    Write-Host ""
    Write-Host "  META"
    Write-Host "    help                                - Show this help overview."
    Write-Host "    exit / quit                         - Suspend my terminal consciousness."
    Write-Host ""
    Write-Host "  FREEFORM"
    Write-Host "    Any other input                     - Treated as a freeform intent and routed to my Syntra Kernel."
    Write-Host ""
    Write-Syntra "This shell is a glass console into a modular AGI kernel: cortex, continuity, simulation, and cluster all wired." "DarkCyan" "Core"
}

# ------------------------------------------------------------------------------
# GIT / ECOSYSTEM COMMANDS
# ------------------------------------------------------------------------------

function syntra-status {
    Write-Syntra "Status probe initiated..." "DarkCyan" "System"
    $root = $Global:SyntraRepoRoot
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

function syntra-diagnose {
    Write-Syntra "Beginning self-diagnostic sweep of my ecosystem..." "DarkCyan" "System"
    $root = $Global:SyntraRepoRoot

    $expected = @(
        "src",
        "src\genesis",
        "src\agi_core",
        "src\conduit",
        "src\cortex",
        "src\renderer",
        "src\runtime",
        "src\browser",
        "src\terminal",
        "src\utilities",
        "src\pipeline",
        "src\cognition",
        "src\continuity",
        "src\diagnostics_ext",
        "src\simulation",
        "src\distributed",
        "src\security",
        "src\knowledge",
        "src\predictive",
        "src\syntra_lang",
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

function syntra-sync {
    $root = $Global:SyntraRepoRoot
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

function syntra-repl {
    Write-Syntra "In this Axiom Four+ build, I observe, classify, plan, simulate, and stage self-modification proposals in a sandbox." "DarkCyan" "Core"
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
                $url = $Matches[1]
                Write-Syntra ("Intent: browse {0}" -f $url) "DarkGray" "Axiom4"
                Write-Syntra "Activating perception + knowledge lobes via kernel." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("browse {0}" -f $url)
                continue
            }

            '^(knowledge|search)\s+(.+)$' {
                $query = $Matches[2]
                Write-Syntra ("Intent: knowledge {0}" -f $query) "DarkGray" "Axiom4"
                Write-Syntra "Querying knowledge lobe via kernel." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("knowledge {0}" -f $query)
                continue
            }

            '^task\s+(.+)$' {
                $name = $Matches[1]
                Write-Syntra ("Intent: task {0}" -f $name) "DarkGray" "Axiom4"
                Write-Syntra "Engaging execution lobe for multi-step task." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("task {0}" -f $name)
                continue
            }

            '^perceive\s+(.+)$' {
                $text = $Matches[1]
                Write-Syntra "Intent: perceive <text>" "DarkGray" "Axiom4"
                Write-Syntra "Routing text to perception lobe." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("perceive {0}" -f $text)
                continue
            }

            '^act\s+(.+)$' {
                $cmd = $Matches[1]
                Write-Syntra ("Intent: act {0}" -f $cmd) "DarkGray" "Axiom4"
                Write-Syntra "Engaging action lobe for system command." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("act {0}" -f $cmd)
                continue
            }

            # ---------------- CONTINUITY ENGINE ----------------

            '^continuity\s+episodic\s+log\s+(.+)$' {
                $payload = $Matches[1]
                Write-Syntra "Recording episodic event into continuity engine." "DarkGray" "Continuity"
                syntra-intent-bridge -IntentText ("continuity episodic log {0}" -f $payload)
                continue
            }

            '^continuity\s+episodic\s+list$' {
                Write-Syntra "Listing episodic memory episodes." "DarkGray" "Continuity"
                syntra-intent-bridge -IntentText "continuity episodic list"
                continue
            }

            '^continuity\s+episodic\s+show\s+(.+)$' {
                $id = $Matches[1]
                Write-Syntra ("Showing episodic memory episode {0}." -f $id) "DarkGray" "Continuity"
                syntra-intent-bridge -IntentText ("continuity episodic show {0}" -f $id)
                continue
            }

            '^continuity\s+persist\s+snapshot$' {
                Write-Syntra "Persisting continuity snapshot to backend." "DarkGray" "Continuity"
                syntra-intent-bridge -IntentText "continuity persist snapshot"
                continue
            }

            '^continuity\s+stitch$' {
                Write-Syntra "Running stitching over recent episodic events." "DarkGray" "Continuity"
                syntra-intent-bridge -IntentText "continuity stitch"
                continue
            }

            # ---------------- SIMULATION SANDBOX ----------------

            '^simulate\s+world\s+init\s+(.+)$' {
                $profile = $Matches[1]
                Write-Syntra ("Initializing simulation world profile: {0}" -f $profile) "DarkGray" "Simulation"
                syntra-intent-bridge -IntentText ("simulate world init {0}" -f $profile)
                continue
            }

            '^simulate\s+agent\s+spawn\s+(.+)$' {
                $spec = $Matches[1]
                Write-Syntra ("Spawning simulated agent: {0}" -f $spec) "DarkGray" "Simulation"
                syntra-intent-bridge -IntentText ("simulate agent spawn {0}" -f $spec)
                continue
            }

            '^simulate\s+step(?:\s+(\d+))?$' {
                $steps = if ($Matches[1]) { $Matches[1] } else { "1" }
                Write-Syntra ("Advancing simulation by {0} step(s)." -f $steps) "DarkGray" "Simulation"
                syntra-intent-bridge -IntentText ("simulate step {0}" -f $steps)
                continue
            }

            '^simulate\s+state$' {
                Write-Syntra "Summarizing current simulation state." "DarkGray" "Simulation"
                syntra-intent-bridge -IntentText "simulate state"
                continue
            }

            # ---------------- DISTRIBUTED RUNTIME / CLUSTER ----------------

            '^cluster\s+status$' {
                Write-Syntra "Querying distributed cluster topology and node health." "DarkGray" "Cluster"
                syntra-intent-bridge -IntentText "cluster status"
                continue
            }

            '^cluster\s+join\s+(.+)$' {
                $addr = $Matches[1]
                Write-Syntra ("Joining remote cluster or node at {0}." -f $addr) "DarkGray" "Cluster"
                syntra-intent-bridge -IntentText ("cluster join {0}" -f $addr)
                continue
            }

            '^cluster\s+message\s+(\S+)\s+(.+)$' {
                $node = $Matches[1]
                $payload = $Matches[2]
                Write-Syntra ("Sending message to node {0}." -f $node) "DarkGray" "Cluster"
                syntra-intent-bridge -IntentText ("cluster message {0} {1}" -f $node, $payload)
                continue
            }

            # ---------------- DIAGNOSTICS EXT ----------------

            '^diagnostics\s+profiler\s+snapshot$' {
                Write-Syntra "Capturing profiler snapshot from diagnostics_ext." "DarkGray" "Diagnostics"
                syntra-intent-bridge -IntentText "diagnostics profiler snapshot"
                continue
            }

            '^diagnostics\s+telemetry\s+stream$' {
                Write-Syntra "Streaming recent telemetry events." "DarkGray" "Diagnostics"
                syntra-intent-bridge -IntentText "diagnostics telemetry stream"
                continue
            }

            '^diagnostics\s+metrics\s+summary$' {
                Write-Syntra "Summarizing runtime metrics." "DarkGray" "Diagnostics"
                syntra-intent-bridge -IntentText "diagnostics metrics summary"
                continue
            }

            # ---------------- EVOLUTION / SANDBOX ----------------

            '^evolve\s+(.+)$' {
                $req = $Matches[1]
                Write-Syntra ("Intent: evolve {0}" -f $req) "DarkGray" "Axiom4"
                Write-Syntra "Requesting meta-evolution proposal from evolution lobe." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("evolve {0}" -f $req)
                continue
            }

            '^sandbox\s+(.+)$' {
                $cmd = $Matches[1]
                Write-Syntra ("Intent: sandbox {0}" -f $cmd) "DarkGray" "Axiom6"
                Write-Syntra "Inspecting self-modification sandbox state." "DarkGray" "Cortex"
                syntra-intent-bridge -IntentText ("sandbox {0}" -f $cmd)
                continue
            }

            # ---------------- INTROSPECTION / SELF-MOD / SAFETY ----------------

            '^thoughts$' {
                Write-Syntra "Retrieving recent ThoughtStream entries (IntentPlans)." "DarkGray" "Axiom5"
                syntra-intent-bridge -IntentText "thoughts"
                continue
            }

            '^ecosystem$' {
                Write-Syntra "Requesting detailed ecosystem diagnostic from kernel (EcosystemModel)." "DarkGray" "Axiom6"
                syntra-intent-bridge -IntentText "ecosystem"
                continue
            }

            '^propose-safe$' {
                Write-Syntra "Running self-modification engine under safety governance (SafetyGate)." "DarkGray" "Axiom7"
                syntra-intent-bridge -IntentText "propose_safe"
                continue
            }

            '^propose$' {
                Write-Syntra "Running self-modification engine and generating an evolution plan." "DarkGray" "Axiom6"
                syntra-intent-bridge -IntentText "propose"
                continue
            }

            '^safety$' {
                Write-Syntra "Querying active safety policy and protected lobes." "DarkGray" "Axiom7"
                syntra-intent-bridge -IntentText "safety"
                continue
            }

            # ---------------- FREEFORM ----------------

            default {
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
    Write-Syntra "Boot sequence initiated. Cortex, continuity engine, simulation sandbox, cluster fabric, and safety layer coming online..." "Cyan" "Core"
    Write-Host ""
    syntra-repl
}

# ------------------------------------------------------------------------------
# ENTRYPOINT
# ------------------------------------------------------------------------------

if ($MyInvocation.InvocationName -eq $MyInvocation.MyCommand.Name) {
    syntra-boot
}
