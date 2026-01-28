<!--
================================================================================================
    SYNTRA KERNEL — TRIAL CONTRIBUTION GUIDE
------------------------------------------------------------------------------------------------
        .\s/.
       :: S ::
        '/s\'

   File:        CONTRIBUTING_TRIALS.md
   Module:      Trial Contribution Guidelines
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Standards and expectations for adding new experimental trials to the Syntra Kernel
                proving grounds.
================================================================================================ 
-->

# Contributing to Syntra Kernel Trials

The `trials/` directory is the experimental frontier of Syntra Kernel.  
To maintain clarity and long‑term value, follow these guidelines when adding new trials.

---

## 1. Use the Trial Template
Start with `TEMPLATE_TRIAL.md` and fill it out completely.  
Every trial must have:
- a clear purpose  
- an axiom level  
- documented observations  
- a defined status  

---

## 2. Document Your Code
Every trial file must include:
- the Syntra sigil  
- a full banner  
- author name  
- description  
- notes  

Trials are historical artifacts — treat them with care.

---

## 3. Update the Manifest
Add your trial to `manifest.toml` with:
- name  
- file  
- axiom level  
- status  
- description  

This keeps the experimental landscape navigable.

---

## 4. Keep Trials Isolated
Trials must not:
- modify core modules  
- introduce breaking changes  
- depend on unstable internal APIs  

They are sandboxes, not production code.

---

## 5. Promote Only When Ready
If a trial proves valuable:
- extract the concept  
- refine it  
- move it into `src/` as a proper module  

Trials are stepping stones, not destinations.
