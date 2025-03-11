---
title: 4a - dApps Polkadot-js (Part 2)
description: Understanding the importance and limitations of Polkadot-JS
duration: 45 minutes - 1 hour
---

# 4a dApps Polkadot-js (Part 2)

---

## Why is a Base Understanding of Polkadot-JS Important?

---

### Polkadot-JS as the Old Industry Standard

- One of the earliest and most comprehensive SDKs for Polkadot
- Widespread adoption as result of years of continued development
- Foundation of ecosystem tooling and infrastructure

Notes:

Polkadot-JS has firmly established itself as the old industry standard for interacting with the Polkadot and Kusama networks. As one of the earliest and most comprehensive SDKs for Polkadot, it has been designed and continuously updated to support almost every iteration of the network's evolving metadata, runtime upgrades, and structural changes.

It has become a fundamental building block for anyone developing on Polkadot, and its widespread use has set the baseline for how developers build, integrate, and deploy Polkadot-based applications. This widespread adoption comes as result of years of continued development and maintenance, which has turned the project into a mature set of developer tools.

This widespread adoption is not just because of its maturity and feature set but because Polkadot-JS simplifies the complex task of interacting with a decentralized, multi-chain network. It once served as the go-to library for developers needing to query chain data, send transactions, and handle Polkadot-specific logic.

---

### Influence on Polkadot-Based Applications

- Many existing tools and legacy systems depend on it
- Future migrations to new tools require knowledge of Polkadot-JS
- Design patterns established by Polkadot-JS remain influential

Notes:

Without a solid base understanding of how Polkadot-JS works, it becomes difficult to effectively develop or maintain applications within the Polkadot ecosystem. Because it's the first point of contact for interacting with Polkadot and Kusama, many other blockchains and ecosystems look to Polkadot-JS for inspiration in their own SDKs and development frameworks.

By understanding Polkadot-JS, developers can more easily transfer their knowledge to other ecosystems or even contribute to the continued evolution of Polkadot-JS itself.

Even as newer tools emerge, understanding the patterns and approaches used in Polkadot-JS helps developers better understand how to interface with Substrate-based chains and provides valuable context for the design decisions made in newer tools.

---

### Efficient Problem-Solving and Debugging

- Understanding its behavior and design is key to troubleshooting
- Knowledge of limitations helps avoid repeating design mistakes
- Provides insight into Substrate architecture and data structures

Notes:

Given that Polkadot-JS is foundational to many applications in the Polkadot ecosystem, understanding its behavior and design is key to troubleshooting and resolving issues. Whether it's debugging failed transactions, understanding chain synchronization problems, or diagnosing issues with the structure of events and extrinsics, a deep knowledge of Polkadot-JS enables developers to quickly pinpoint the source of problems and fix them efficiently.

Since Polkadot-JS is the tool used for many underlying blockchain interactions, errors and issues that arise in other applications or tools often trace back to how Polkadot-JS is being utilized.

Understanding the limitations and flaws of Polkadot-JS also helps avoid these mistakes when developing new solutions. Using Polkadot-JS as the base on which to build a new, more efficient and flexible solution works better when keeping in mind these limitations.

---

### Knowledge Transfer to Future Tools

- Understanding Polkadot-JS helps with migration to newer tools like PAPI
- Core concepts of chain interaction remain consistent across tools
- Historical context provides insight into why newer tools make different choices

Notes:

As the ecosystem evolves and newer tools like PAPI (Polkadot API) emerge, understanding Polkadot-JS becomes even more valuable. The knowledge of how Polkadot-JS interacts with Substrate chains transfers directly to these newer tools, making the transition smoother.

The core concepts of chain interaction - connecting to nodes, querying state, constructing and submitting transactions - remain consistent across different tools, though the implementation details may change.

Understanding why Polkadot-JS made certain design choices, and where those choices led to limitations, provides valuable context for appreciating the different approaches taken by newer tools. This historical perspective helps developers make more informed decisions about which tools to use for different projects and use cases.

---

## When Might You Want to Use Polkadot-JS?

---

### Historical Metadata Queries

- Essential for analyzing past chain state
- Comprehensive support for all historical runtime versions
- Crucial for block explorers and data analysis tools

Notes:

One of the standout features of Polkadot-JS is its comprehensive support for historical metadata. This makes it the tool of choice for applications that need to analyze or display data from any point in the chain's history.

Block explorers, data analysis tools, and historical auditing applications all benefit from this capability. No other tool currently provides the same level of historical compatibility, making Polkadot-JS irreplaceable for these use cases.

As chains evolve and undergo runtime upgrades, the structure of their data and the methods for interacting with them change. Polkadot-JS maintains backward compatibility with all these changes, allowing developers to decode and interpret data from any block height.

---

### As a Reference or Utility

- Useful for learning Substrate interactions before implementing custom solutions
- Provides working examples of common blockchain operations
- Source code serves as documentation for Substrate interaction patterns

Notes:

Even when building custom solutions, Polkadot-JS serves as an invaluable reference. Its source code effectively documents how to interact with Substrate chains at a low level, making it easier to understand the underlying protocols and data structures. That being said, there are definitely exceptions (rpc interface, extension interface, etc).

Many developers start by prototyping their applications with Polkadot-JS before implementing more specialized solutions. This approach allows them to validate their concepts quickly before investing in custom development.

The various utilities provided by Polkadot-JS, particularly in the @polkadot/util and @polkadot/util-crypto packages, remain useful even when building applications that don't use the full API. These well-tested utilities save developers from having to reimplement common cryptographic and formatting functions.

---

### Legacy System Maintenance

- Many existing tools still rely on Polkadot-JS
- Understanding the library is crucial for maintaining these systems
- Required for debugging issues in Polkadot-JS-dependent applications

Notes:

A significant portion of the Polkadot ecosystem was built with Polkadot-JS, and these systems will continue to require maintenance for years to come. Understanding Polkadot-JS is essential for developers tasked with maintaining or updating these existing applications.

When issues arise in these systems, debugging often requires a deep knowledge of how Polkadot-JS interacts with the blockchain. Without this understanding, diagnosing and fixing problems becomes significantly more challenging.

Even as new applications increasingly adopt newer tools, the legacy of Polkadot-JS will persist in the ecosystem for the foreseeable future, ensuring that expertise in this library remains valuable.

---

### For Comprehensive Development UIs

- Polkadot.js Apps remains the most feature-complete UI for chain interaction
- Provides a sandbox for testing chain functionality
- Useful for debugging and verifying on-chain behavior

Notes:

The Polkadot.js Apps interface remains one of the most comprehensive UIs for interacting with Substrate-based chains. It provides access to virtually every aspect of chain functionality, from basic transfers to complex governance operations.

This interface serves as an invaluable sandbox for testing and debugging chain functionality. Developers can use it to verify that their on-chain logic works as expected before integrating it into their own applications.

While more specialized and user-friendly interfaces exist for specific chains and use cases, none match the comprehensiveness of Polkadot.js Apps for development and debugging purposes.

---

### Cross-Chain Development

- Consistent interface across different Substrate chains
- Reduces friction when working with multiple chains
- Simplifies development of cross-chain applications

Notes:

For developers working with multiple Substrate-based chains, Polkadot-JS provides a consistent interface that significantly reduces the learning curve for each new chain. Once familiar with the API patterns, developers can apply them across any Substrate chain with minimal adjustments.

This consistency is particularly valuable for cross-chain applications that need to interact with multiple networks. Rather than learning entirely different APIs for each chain, developers can leverage their existing knowledge of Polkadot-JS.

The unified approach also simplifies the codebase for cross-chain applications, as much of the logic for chain interaction can be shared across different networks.

---

## Limitations That Emerged Over Time

---

### Poor Light Client Support

- Heavy reliance on centralized nodes for queries
- Limited support for true decentralized applications
- Challenges for mobile and resource-constrained environments

Notes:

One of the most significant limitations of Polkadot-JS is its poor support for light clients. The library is designed to work with full RPC nodes, which typically means relying on centralized infrastructure providers.

This dependency on centralized nodes undermines the decentralized nature of blockchain applications. If these nodes go down or are compromised, applications built on Polkadot-JS may become unavailable or vulnerable.

The lack of efficient light client support also creates challenges for mobile applications and other resource-constrained environments, where downloading and processing large amounts of blockchain data is impractical.

Newer tools and approaches are addressing this limitation by implementing more efficient light client protocols and WASM-based verification that can run directly in browsers and mobile devices.

---

### Overly Coupled Dependencies

- Tightly interconnected packages
- Difficult to use individual components
- Increased bundle sizes for web applications
- Challenges in maintenance and updates

Notes:

Polkadot-JS consists of numerous packages that are tightly coupled with each other. This high level of interdependency means that using any single component often requires pulling in large portions of the entire library.

For web applications, this results in larger bundle sizes, which can negatively impact loading times and performance. Even simple applications may end up including substantial amounts of unused code.

The tight coupling also complicates maintenance and updates. Changes to one package often necessitate updates to several others, creating a complex web of dependencies that must be managed carefully.

This architecture made sense in the early days of the ecosystem when rapid development was prioritized over modularity, but it has become increasingly problematic as the ecosystem has matured and more specialized use cases have emerged.

---

### Older JSON-RPC Interface

- Not optimized for modern application needs
- Inefficient data transfers
- Limited batching capabilities
- Challenges with real-time updates and subscriptions

Notes:

Polkadot-JS was built around the original JSON-RPC interface for Substrate, which wasn't designed with modern application needs in mind. This older interface has several limitations:

1. Inefficient data transfers: The JSON format is verbose, leading to larger payloads than necessary for many operations.

2. Limited batching capabilities: While some batching is supported, it's not as efficient or flexible as newer approaches.

3. Challenges with real-time updates: While the interface supports subscriptions, managing them effectively at scale can be complex.

4. Not optimized for partial data retrieval: Applications often need to retrieve specific pieces of data, but the RPC interface may return more information than necessary.

Newer interfaces and protocols are addressing these limitations with more efficient data encoding, better support for partial data retrieval, and improved subscription models.

---

### Too Many Responsibilities

- Acts as a wallet, RPC client, API, and UI tool
- Jack of all trades, master of none
- Difficult to optimize for specific use cases
- Challenging to maintain and evolve

Notes:

Polkadot-JS attempts to fulfill too many diverse functions, from low-level cryptography to high-level UI components. This broad scope makes it difficult to excel in any single area.

For developers with specific needs, this means either accepting suboptimal solutions or building custom implementations. Neither option is ideal, especially for teams with limited resources.

The wide range of responsibilities also complicates maintenance and evolution. Improvements to one aspect of the library might introduce regressions in others, making it challenging to implement significant changes without extensive testing and coordination.

Modern software development generally favors more focused, modular tools that do one thing well and can be combined as needed. Newer tools in the ecosystem are following this approach, providing more specialized solutions for specific use cases.

---

### High Abstraction Complexity

- Many layers of abstraction
- Difficult debugging process
- Steep learning curve for new developers
- Challenges in understanding error sources

Notes:

Polkadot-JS employs multiple layers of abstraction to simplify common tasks and hide the complexity of the underlying blockchain protocols. While these abstractions make simple tasks easier, they can make debugging and troubleshooting significantly more complex.

When errors occur, they often happen deep within the library's internals, making it difficult to identify the root cause. The error messages may be cryptic or provide limited information about what went wrong.

This complexity creates a steep learning curve for new developers. Understanding how all the pieces fit together requires significant investment of time and effort, which can be a barrier to entry for those new to the ecosystem.

Even experienced developers may struggle to understand the full implications of certain API calls or the exact data flow through the various abstraction layers.

---

### Handling of Type Information

- Relies on dynamic runtime metadata
- Can be brittle when chains update
- Challenges with type consistency across runtime versions
- Complex type registry management

Notes:

Substrate chains provide type information through runtime metadata, which Polkadot-JS uses to interpret the chain's data structures. This approach is flexible but introduces several challenges:

1. Type information is loaded dynamically at runtime, which can lead to errors if the metadata doesn't match the chain's current state.

2. When chains update their types, applications may need to manually update their type definitions to maintain compatibility.

3. Managing custom types across different runtime versions can become complex, especially for applications that need to work with multiple chains or historical data.

4. The type system is intricate and can be difficult to debug when issues arise.

Newer approaches are exploring ways to make type handling more robust and developer-friendly, such as generating static TypeScript definitions from chain metadata or using more structured type systems.

---

## Future of Polkadot-JS

- Continued maintenance for legacy support
- Gradual transition to more modern tooling (e.g., PAPI)
- Community-driven improvements and bug fixes
- Reference implementation for new development patterns

Notes:

While newer tools are emerging to address some of Polkadot-JS's limitations, it will continue to be maintained for the foreseeable future due to its widespread adoption and the many systems that depend on it.

The knowledge and patterns developed in Polkadot-JS will inform the next generation of Substrate development tools, ensuring that its legacy continues even as newer alternatives gain adoption.

For developers entering the ecosystem today, understanding Polkadot-JS provides valuable context about how Substrate chains work, even if they ultimately build with newer tools.

---

## Key Takeaways

<pba-flex center>

- Polkadot-JS has been the cornerstone of Substrate development
- Knowledge of Polkadot-JS remains valuable for the ecosystem
- Understanding its limitations informs better tooling design
- Many existing tools and systems depend on Polkadot-JS
- Historical metadata support remains unmatched

</pba-flex>

Notes:

Polkadot-JS has played a crucial role in the growth and development of the Polkadot ecosystem. While it has limitations, its comprehensive functionality and historical support make it an invaluable tool for many development scenarios.

As the ecosystem evolves, developers should understand both when to use Polkadot-JS and when newer tools might be more appropriate for their specific use cases. This balanced understanding will ensure they can make the best technology choices while building on Substrate-based chains.

The knowledge gained from working with Polkadot-JS – understanding chain metadata, transaction construction, and blockchain state management – transfers well to other tooling and provides a solid foundation for Substrate development regardless of the specific libraries used.

---

## Modern Alternatives

- **PAPI** (Polkadot API) - Next-generation TypeScript SDK with improved modularity
- **Subxt** - Rust-based SDK with strong type safety
- **Substrate Connect** - Light client focused solution for browser environments
- **Custom RPC clients** - Specialized solutions for specific use cases

Notes:

As the ecosystem matures, several alternatives to Polkadot-JS are emerging, each addressing different limitations:

PAPI is being developed as a more modular, modern TypeScript SDK with cleaner separation of concerns and better support for light clients. It aims to provide a more maintainable and extensible foundation for JavaScript/TypeScript applications.

Subxt offers a Rust-based alternative with strong type safety and efficient performance. It's particularly well-suited for backend services and applications where performance is critical.

Substrate Connect focuses specifically on light client functionality, enabling truly decentralized applications that don't rely on centralized RPC endpoints.

Many teams are also developing custom RPC clients tailored to their specific needs, especially for applications with unique requirements or performance constraints.

Understanding the strengths and limitations of Polkadot-JS helps developers make informed decisions about when to use these alternatives and how to effectively migrate to them when appropriate.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions?