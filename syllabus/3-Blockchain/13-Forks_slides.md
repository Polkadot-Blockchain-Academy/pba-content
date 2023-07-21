---
title: Blockchain Forks # Also update the h1 header on the first slide to the same name
description: Detailed Classification for Blockchain Forks Types
duration: 60 minutes
# PBA has a theme: "reveal-md/PBA-theme.css", alternatively, you can use a named default like "night" from this list: https://github.com/hakimel/reveal.js/tree/master/css/theme/source
# Add custom css files for your slides here, comma separated:
separator: "\r?\n---\r?\n"
verticalSeparator: "\r?\n---v\r?\n"
# Below can be any of these: https://revealjs.com/config/
revealOptions:
    transition: "none" # animation between slides = none/fade/slide/convex/concave/zoom
	backgroundTransition: "fade" # background swap between slides = none/fade/slide/convex/concave/zoom
	slideNumber: true
	controls: true
	progress: true
---

# Blockchain Forks

---

# Landscape

---v
## Landscape

#### Ideal World

In an ideal world blockchains would look like this:
<br><br>

<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/no_fork.drawio.svg" />

---v
## Landscape

#### Real World

Things not always go according to plan:

<br>

<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_small.drawio.svg" />

---v
## Landscape

#### Chaotic Real World

And sometimes they get extra messy:

<br>

<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_chaos.drawio.svg" />

---

# What's the goal?

---v

## What's the goal?

#### _Fork Identification_

<img style="width: 500px" src="../../assets/img/3-Blockchain/forks/forks_and_boxes.drawio.svg" />

---v

## What's the goal?

#### _Fork Categorisation_

<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/forks_in_boxes.drawio.svg" />

Notes:

Why? Forks in the same category will exhibit similar behaviour and will require similar handling. Then when making changs we can easily figure out to which box the change belongs and react accordingly.

---

# Fork Categorisation

---v

## Fork Categorisation

#### _Forks Family Tree_

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_family.drawio.svg" />

---

# Transitory Forks

---v

## Fork Categorisation

#### _Transitory Forks_

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_family_transitory.drawio.svg" />

---v

## Transitory Forks

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/transitory_forks.drawio.svg" />

---

# Consensus Forks

---v

## Fork Categorisation

#### _Consensus Forks_

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_family_consensus.drawio.svg" />

---v

# Consensus Forks

## _Validity Set_

---v

## Consensus Forks

#### _Validity Set_
<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/BTC_block.drawio.svg" />

---v

## Consensus Forks

#### _Validity Set_
<br>
<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/BTC_header.drawio.svg" />


---v

## Consensus Forks

#### _Validity Set_
<br>
<img style="width: 500px" src="../../assets/img/3-Blockchain/forks/BTC_header_constraints.drawio.svg" />

---v

## Consensus Forks

#### _Validity Set_
<br>
<img style="width: 500px" src="../../assets/img/3-Blockchain/forks/validity_set.drawio.svg" />

---v

## Consensus Forks

#### _Validity Set_
<br>
<img style="width: 500px" src="../../assets/img/3-Blockchain/forks/universal_set.drawio.svg" />

---v

## Consensus Forks

#### _Validity Set_
<br>
<img style="width: 500px" src="../../assets/img/3-Blockchain/forks/validity_set_old.drawio.svg" />

---v

## Consensus Forks

#### _Validity Set_
<br>
<img style="width: 500px" src="../../assets/img/3-Blockchain/forks/validity_set_new.drawio.svg" />

---v
## Consensus Forks

#### _Validity Set_

<pba-cols>
    <pba-col>
		<img style="width: 500px" src="../../assets/img/3-Blockchain/forks/validity_set_new.drawio.svg" />
    </pba-col>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_soft.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
</pba-cols>

---

# Soft Forks

---v

## Fork Categorisation

#### _Soft Forks_

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_family_soft.drawio.svg" />

---v

## Fork Categorisation

#### _Soft Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_soft.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
    <pba-col>
		<ul>
			<li>Backwards Compatible</li>
			<li>By making the consensus rules more restrictive the set of valid blocks gets smaller.</li>
			<li>Not every (and often none) of the blocks produced under old rules will get accepted by new nodes.</li>
		</ul>
    </pba-col>
</pba-cols>

Notes:
Is decreasing or increasing blocksize a soft fork?

---v

## Fork Categorisation

#### _Soft Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_soft.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
    <pba-col>
		<strong>Examples:</strong>
		<br><br>
		<ul>
			<li>Decreasing blocksize</li>
			<li>Accepting only even/odd hashes</li>
			<li>Disallowing some transaction types</li>
		</ul>
    </pba-col>
</pba-cols>

---v

## Fork Categorisation

#### _Soft Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_soft.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
    <pba-col>
		<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/soft_forks_s50.drawio.svg" />
    </pba-col>
</pba-cols>


---v

## Fork Categorisation

#### _Soft Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_soft.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
    <pba-col>
		<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/soft_forks_g50.drawio.svg" />
    </pba-col>
</pba-cols>

---

# Hidden Forks

---v

## Fork Categorisation

#### _Hidden Forks_

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_family_hidden.drawio.svg" />

---v

## Fork Categorisation

#### _Hidden Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_hidden.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
    <pba-col>
		<ul>
			<li>Conflictless</li>
			<li>The old, now excluded blocks were allowed but never used in practice.</li>
			<li>New nodes are theoretically stricter but practically accept all old blocks.</li>
			<li>Old nodes accept new blocks.</li>
		</ul>
    </pba-col>
</pba-cols>

---v

## Fork Categorisation

#### _Hidden Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_hidden.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
    <pba-col>
		<strong>Examples:</strong>
		<br><br>
		<ul>
			<li>Assigning non-conflicting uses to empty opcodes.</li>
			<li>BTC Ordinals using empty opcodes to implement BTC NFTs.</li>
		</ul>
    </pba-col>
</pba-cols>

---v

## Fork Categorisation

#### _Hidden Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_hidden.drawio.svg" />
		<div style="font-size: 50px;">N ⊆ O</div>
    </pba-col>
    <pba-col>
		<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/soft_forks_hidden.drawio.svg" />
    </pba-col>
</pba-cols>

---
# Hard Forks

---v

## Fork Categorisation

#### _Hard Forks_

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_family_hard.drawio.svg" />

---v
## Fork Categorisation

#### _Hard Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_hard.drawio.svg" />
		<div style="font-size: 50px;">O ⊆ N</div>
    </pba-col>
    <pba-col>
		<ul>
			<li>Forwards Compatible</li>
			<li>By making the consensus rules less restrictive the set of valid blocks gets bigger.</li>
			<li>Not every (and often none) of the blocks produced under new rules will be accepted by the old nodes.</li>
			<li>Every block produced under old rules will get accepted by new nodes.</li>
		</ul>
    </pba-col>
</pba-cols>

---v
## Fork Categorisation

#### _Hard Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_hard.drawio.svg" />
		<div style="font-size: 50px;">O ⊆ N</div>
    </pba-col>
    <pba-col>
		<strong>Examples:</strong>
		<br><br>
		<ul>
			<li>Increasing blocksize</li>
			<li>BTC Cash fork at first*</li>
			<li>Adding new transaction types</li>
			<li>Increasing max nonce value</li>
		</ul>
    </pba-col>
</pba-cols>

---v
## Fork Categorisation

#### _Hard Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_hard.drawio.svg" />
		<div style="font-size: 50px;">O ⊆ N</div>
    </pba-col>
    <pba-col>
			<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/hard_forks_s50.drawio.svg" />
    </pba-col>
</pba-cols>

---v
## Fork Categorisation

#### _Hard Forks_

<pba-cols>
    <pba-col>
		<img style="width: 300px" src="../../assets/img/3-Blockchain/forks/venn_hard.drawio.svg" />
		<div style="font-size: 50px;">O ⊆ N</div>
    </pba-col>
    <pba-col>
			<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/hard_forks_g50.drawio.svg" />
    </pba-col>
</pba-cols>

---

# Full Forks

---v

## Fork Categorisation

#### _Full Forks_

<br>
<img style="width: 800px" src="../../assets/img/3-Blockchain/forks/fork_family_full.drawio.svg" />

---v

## Fork Categorisation

#### _Full Forks_

<pba-cols>
    <pba-col>
		<img style="width: 200px" src="../../assets/img/3-Blockchain/forks/venn_full.drawio.svg" />
		<div style="font-size: 50px;">O ∩ N = ∅</div>
    </pba-col>
    <pba-col>
		<ul>
			<li>Fully Incompatible</li>
			<li>Soft + Hard</li>
			<li>By changing the consensus rules the sets can become disjoint or overlapping.</li>
			<li>Most (and often all) blocks produced under one ruleset are not accepted under the other.</li>
		</ul>
    </pba-col>
</pba-cols>

---v
## Fork Categorisation

#### _Full Forks_

<pba-cols>
    <pba-col>
		<img style="width: 200px" src="../../assets/img/3-Blockchain/forks/venn_full.drawio.svg" />
		<div style="font-size: 50px;">O ∩ N = ∅</div>
    </pba-col>
    <pba-col>
		<strong>Examples:</strong>
		<br><br>
		<ul>
			<li>Changing the hashing function</li>
			<li>Changing the signature scheme</li>
			<li>Specific combinations of soft and hard forks</li>
			<li>BTC Cash fork in the end*</li>
		</ul>
    </pba-col>
</pba-cols>

---v

## Fork Categorisation

#### _Full Forks_

<pba-cols>
    <pba-col>
		<img style="width: 200px" src="../../assets/img/3-Blockchain/forks/venn_full.drawio.svg" />
		<div style="font-size: 50px;">O ∩ N = ∅</div>
    </pba-col>
    <pba-col>
		<img style="width: 600px" src="../../assets/img/3-Blockchain/forks/full_forks__&_50.drawio.svg" />
    </pba-col>
</pba-cols>

---

## Summary


<pba-cols>
    <pba-col>
		<img style="width: 400px" src="../../assets/img/3-Blockchain/forks/soft_forks_s50.drawio.svg" />
		<br>
		<img style="width: 400px" src="../../assets/img/3-Blockchain/forks/soft_forks_g50.drawio.svg" />
    </pba-col>
	<pba-col>
		<img style="width: 400px" src="../../assets/img/3-Blockchain/forks/hard_forks_s50.drawio.svg" />
		<br>
		<img style="width: 400px" src="../../assets/img/3-Blockchain/forks/hard_forks_g50.drawio.svg" />
    </pba-col>
    <pba-col>
		<img style="width: 400px" src="../../assets/img/3-Blockchain/forks/full_forks__&_50.drawio.svg" />
    </pba-col>
</pba-cols>

Notes:
- Bitcoin cash pivot from hard to full because they didn't have enough HP.
- Soft are often preferred for changes because with >50%HP they do not fracture the community (BTC community logic)
- Hard can be preferred as they seem to better represent minorities. If some people don't agree with the majority they naturally fork off and are not peer pressured to follow (ETH community logic)

---

# Thank you!

---

<img style="width: 1800px" src="../../assets/img/3-Blockchain/forks/forks.drawio.svg" />

