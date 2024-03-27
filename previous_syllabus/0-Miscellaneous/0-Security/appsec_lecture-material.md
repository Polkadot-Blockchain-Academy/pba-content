# Appsec Lecturer Materials

- Security is a matter of balance, not too much, not less. Only adequate.
- Security is about your residual risks, not what you have prevented.

**1. Securing SDLC**

**1.1. The Big Picture of AppSec**

<!-- ![](https://hackmd.io/_uploads/SkZLpbmYh.png) -->
<!-- TODO FIXME get this file local!! 405 code! -->

**Security needs to be enforced through the use of controls. Controls must be:**

- Designed
- Developed
- Implemented
- Configured
- Operated
- Monitored
- Improved

**How do we decide on Controls?**

**RISK:**

The likelihood of a threat exploiting a vulnerability and thereby causing damage to an asset.

**1.1.1. A brief look at core principles**

In case you have yet to hear of the CIA Triad.

**Confidentiality**
Protect sensitive data from improper disclosure (Intentional or accidental)

- Privacy (more related to individuals)
- Secrecy (more related to business, military, IP)

**How to protect confidentiality**

- Masking
- Obfuscation
- Tokenization
- Encryption

In summary, **ensure that only authorized people can access their authorized entities.**

**Integrity**
Protection of data and processes from accidental or intentional alteration or modifications

- Accuracy
- Completeness
- Precision
- Reliability
- Authenticity

**How to protect integrity**

- Hashing
- HMAC (aka keyed hashing)
- Digital Signatures (DSS, ECC etc)
- Code Signing

In summary, **ensure that only authorized changes are made by authorized entities.**

**Availability**
Protection of data to make it available when required

- Systems
- Information
- Networks
- Personnel
- Software

**Availability within Applications**

- Resilience (Withstand the attacks they will face)
- Error handling
- Scalability
- Number of users
- Transaction volume etc

In summary, **ensure that the data will always be available when it is required. And business itself should decide the SLA, not the technical people.**

**The AAA (Triple A)**

- **Authentication (AuthN)**

  - Validation (Validate, verify, prove) of the ID provided
  - Rightful owner of the ID (2/3)
    - Something you know
    - Something you have
    - Something you are

- **Authorization (AuthZ)**

  - Correct privilege levels of provided to subject
  - Access Controls are the centre of the attention
  - To permit authorized entities (persons, processes) to perform authorized functions

- **Accountability**

  - Ability to identify all entities involved
  - Tracking and recording all(?) activity

- **Non Repudiation**
  - Ability to prevent a subject from denying a previous action with an object in a system

**1.1.2. A brief look at design principles**

- **Good enough security**

  - Don’t spend $10.000 on a safe to protect a $20 bill

- **Least Privilege**

  - Don't give your safe's key to everybody, give only what they need

- **Separation of Duties**

  - Don't give the power of creating invoices, approving invoices and sending money to one person

- **Defense in Depth**

  - A castle has a moat, thick walls, restricted access points, high points for defense, multiple checkpoints inside etc.; what do you have?

- **Fail-Safe**

  - Any function that is not specifically authorized is denied by default

- **Economy of Mechanism**

  - Security is already a complex topic, don’t make it more complicated (KISS)

- **Complete Mediation**

  - Every critical operation must have verification at every time.

- **Open Design**

  - Don't even try: Security over obscurity

- **Least Common Mechanism**

  - is like the rarest key that opens specific locks, not used often but still can cause significant damage when it does.

- **Psychological Acceptability**

  - There is no point if users cannot use your security controls seamlessly.

- **Weakest Link**

  - A chain is only as strong as its weakest link (Booooo)

- **Leverage Existing Components**

  - Fewer components, fewer attack surface, but more;

- **Single Point of Failure**
  - If SPoF fails, means the entire system fails

**1.1.3. Requirements**

- Business Requirements
- Legal Requirements
  - Compliance
- Ethical Requirements
- Security  Requirements

  **Functional Requirements**

- What the system will do

**Non-Functional Requirements (Security)**

- How the system will do

**Benchmarking**

- The way of doing that is by looking to other companies to see what they are doing
  - Same industry sector
  - Same region etc.

**1.1.4. Architecture & Design**

- Identify Attack Surfaces
  - What potential surfaces do you have?
- Identify Attack Vectors
  - What potential vectors do you have?
- Allocate Security Controls
  - Risk based approach
  - Security Controls

**Security controls can be;**

- **Directive (Safeguard \[Proactive\] - means before the incident)**
  - The policy is an example. This is what you are allowed to do, or you are not allowed to do

**Security controls can be;**

- **Deterrent (Safeguard \[Proactive\] - means before the incident)**
  - Discourage somebody from doing something wrong. For ex. watching people with a security camera. Once they know they are under observation, they will hesitate.

**Security controls can be;**

- **Preventive (Safeguard \[Proactive\] - means before the incident)**
  - Try to stop a person from doing something wrong. For ex. Password is a preventive control.

**Security controls can be;**

- **Detective (Countermeasures \[Reactive\] - means in the incident moment or afterwards)**
  - Trying to detect an incident. For ex. logs

**Security controls can be;**

- **Corrective (Countermeasures \[Reactive\] - means afterwards)**
  - Tries to reestablish control after an incident and correct the immediate problem.

**Security controls can be;**

- **Restoration/Recovery (Countermeasures \[Reactive\] - means afterwards)**
  - Try to rebuild and get back to normal.

**1.1.5. Implementation**

**Secure Coding Practices**

- **Coding Standards**
  The adoption of coding standards is essential.
  _ Language-specific standards
  _ Avoidance of weak procedures
  _ Usage of commonly used modules
  _ Readability and maintainability \* Documentation

**Document Relevant Coding Practices**

- Coding Standards
- Coding Guidelines
- Regulatory Requirements
- Encryption algorithm implementations
- Logging

**Separation of Development and Operational Environment**
**We need**

- Protection of executable code
- Separate development environment
  - Also, do separate QA or test environment
- Version control
- Regression

In summary, **The adaptation, communication and enforcement of coding are essential to increasing the quality of programming practices.**

**1.1.6. Testing**

- **Objective:** Develop test cases for all possible scenarios (normal and abnormal usage, environmental and operational tests).
- **Importance of Regression testing:** It ensures that new features do not break other functionalities.

- **Attack Surface Validation and Discovery**

  - **Process:** Review and validate previously documented attack surfaces, and discover new ones.
  - **Focus areas:** Inputs, outputs, architecture, networks, users, administrators, change control, and error handling.

- **Types of Application Testing**

  - Normal user activity and errors.
  - **Failure testing:** Fault injection, cancellation of transaction, invalid input.
  - **Capacity testing**: Load, stress, break, and transaction capacity tests.

- **Testing Box and Approaches**

  - **Concept of 'Testing Box':** Consider the application as a box and test its strength, usefulness, sealing, manufacturing process, and environment adaptability.
  - **Approaches:** Black Box Testing, Interactive Application Security Testing (IAST), and White Box Testing.

- **Importance of Peer Review**

  - **Benefits:** Adherence to coding standards, consistency, training/mentoring of developers.
  - **Challenges:** Personality conflicts, personal opinions, time, independence, and skill of the assessor.

- **Vulnerability Assessment and Penetration Testing**

  - **Vulnerability Assessments:** Systematic examination of the system to identify security deficiencies.
  - **Penetration Testing:** A test methodology to circumvent or defeat the security features of a system.
  - **Hats in Testing:** White hat (Ethical), Grey hat (Semi-ethical), Black hat (Unauthorized, Non-ethical).

- **System Scanning and Privacy**

  - Scanning: Involves querying, poking, probing, testing, and observing the system to discover deficiencies.
  - Privacy concerns: Availability of sensitive data to unauthorized personnel.

- **Cryptography and its Challenges**

  - **Importance of Cryptography:** Secure key distribution and generation, secure key storage, and Random Number Generators.
  - **Challenges:** Lost keys, crypto-period, hard-coded credentials.

- **Ongoing Testing**

  - Testing as an ongoing activity: Repeatable and consistent test cases, comparing results from different test cycles, re-testing after corrections.
  - Support for Testing: Audit hooks, test cases, logs.
  - Fuzzing: Testing the 'edges' using generated and mutated data.
  - Synthetic Transactions: Simulating the actions of a user for constant health checks.

- **Documentation, Verification, and Validation**

  - **Objective:** Testing should discover flaws or deficiencies in controls.
  - **Verification:** Ensures all security controls documented in the design are built into the final application.
  - **Validation:** Check if the implemented security controls are correct to mitigate the risk.

- **Control Testing**

  - Three Steps:
    - Verify that each control was implemented as designed.
    - Verify that the control is operating correctly.
    - Validate that the control is accomplishing its intended function - mitigating the risk.

- **Known Common Problems**

  - Use sources like OWASP, CVSS, and CWE to be aware of and mitigate common vulnerabilities.

- **Requirement Traceability Matrix (RTM)**

  - Document test results, identify errors, and update the RTM to show completed tests and test status.

- **Classifying and Tracking Security Errors**

  - All test results should be tracked, and deficiencies identified.
  - Decisions to be made on breaking builds (Continuous Integration/Continuous Deployment).

- **Ownership and Resolution of Security Deficiencies**

  - Every security deficiency should have an owner responsible for addressing it.
  - Addressing deficiencies depends on the severity, time, budget, other mitigating controls, and availability of solutions.

- **Re-testing and Risk Acceptance**

  - After resolving problems, re-testing is necessary for satisfactory results.
  - Risk Acceptance: The level of risk a manager is willing to accept on behalf of the organization, considering factors like cost-benefit, time, budget, need for implementation, and compensating controls.

- **Documentation**

  - Importance of documentation: User guides, Installation guides, Operational guides, and System Security Plan (SSP).
  - Undocumented Functionality: Look out for undocumented features, unauthorized functions, trap doors, maintenance hooks, and outgoing data streams.

- **Support for Audit**
  - Audit hooks, Logs, Reports, Integrated test, Facility, and Synthetic transactions for maintaining a robust audit system.

**2. Components of application security**

- **Threat Modelling**

  - Identify your threats, before others do
  - A structured approach to identify, quantify, and address the security threats and risks associated with an application
  - An early-stage activity is used to define security requirements for design (in theory 🙂)
  - “What could happen if a malicious user does this?”
  - Joint effort between developers, security professionals and relevant stakeholders

- **2 Main Approaches for threat modelling:**

  - Manual Threat Modelling
    - Create data flow diagram that shows how data will flow
    - A software bill of materials (SBOM) that provides the components that application uses
  - Manual Threat Modelling
    - Web service integration points such as APIs and other 3rd party systems
    - Popular techniques: STRIDE, PASTA, OCTAVE, and VAST

- **2 Main Approaches for threat modelling:**

  - Threat modelling using a tool
    - Popular tools: Threat Dragon by OWASP, MS Threat modelling tool, SecuriCAD, ThreatModeler, IriusRisk

- **Security Analysis Tools**

  - **SAST (Static Application Security Testing)**
    - SAST tools perform analysis on the existing code, aiming to identify potential security vulnerabilities.
    - Example: find issues like hardcoded passwords in plain text within the source code.
    - The method used by SAST involves techniques such as taint analysis and data flow analysis.
    - Taint analysis: This process involves tracing user input across the application to check if it is properly sanitised before being used.
    - Data flow analysis: This technique involves the collection of run-time data while the code remains static.
    - SAST (Static Application Security Testing) tools are commonly utilized during the development phase to detect and address issues early in the development lifecycle
    - A significant number of current SAST providers offer integrated development environment (IDE) plugins.
    - These plugins enable developers to conduct scans as the code is being written.
  - Be careful with;
    - SAST tools are usually bound by limitations in the language that they cover.
    - There is no one tool to rule them all.
    - Understand your needs before buying expensive tools.
    - SAST tools produce many false positives.
      - You need to reduce the noise by tuning them
    - Always remember, those are static tools, which means cannot see how it runs in an environment.

- **DAST (Dynamic Application Security Testing)**

  - DAST tools analyse applications in a running environment, similar to automated penetration testing.
  - They aim to identify vulnerabilities like parameter tampering, cross-site scripting, and improper redirects.
  - Penetration testers frequently use DAST tools to detect easily exploitable weaknesses, thus reducing manual testing workload.
  - DAST tools operate independently of the technology as they assess the running application for vulnerabilities.
  - Example: These tools adopt an outside-in approach, focusing on HTTP conversations, indifferent to the underlying language or framework.
  - Be careful;
    - DAST tools cannot tell you the line of code where an issue is found. (unless it has been instrumented into the application)
    - DAST is typically used later in the development lifecycle, resulting in later detection of issues compared to SAST.
  - Be careful;
    - Running DAST in a developer's local environment is uncommon, though possible.
    - DAST doesn't identify code-specific issues, like hardcoded passwords.
    - DAST's findings require expert analysis to confirm their validity.

- **IAST (Interactive Application Security Testing)**

  - IAST is a combination of SAST and DAST.
  - Assesses the application from within through instrumenting the code.
  - Example: The vendor supplies a library incorporated into the application build, enabling IAST tool operation within the app
  - This inclusion allows IAST tools to access code, HTTP conversations, library data, backend connections, and configuration information.
  - Be careful;
    - IAST tools may require actual attack scenarios to detect vulnerabilities, which can be a drawback.
    - In the absence of comprehensive testing, vulnerabilities may remain unnoticed until a specific feature is used.
    - For instance, infrequently run functions, like certain reporting tasks, could carry unnoticed vulnerabilities to production.

- **Software Composition Analysis**

  - Let's agree that a small percentage of code is actually written by a developer from scratch. They use;
    - Libraries
    - Packages etc
  - If you try to create a project that handles math equations, there is already a secure and reliable library for this.
    - This gave us convenience but also created friction in managing the sprawl of libraries used in an overall project. How do you know the libraries;
      - are secure
      - not breaching any licence etc.
  - The answer is the SCA tool.
  - Software Composition Analysis (SCA) tools (usually) manage the use of open-source components and license tracking.
  - They scan an application's code base, including related items like containers and registries.
  - They identify all open-source components, helping to create a software bill of materials.
  - SCA tools provide licence compliance data and detect potential security vulnerabilities.
  - Some tools even prioritise and automatically remediate open-source vulnerabilities.
  - Be careful;
    - About their potential to flag a library as insecure after it's already in production.
    - Might require upgrading other libraries or making significant architectural changes to address identified issues.
    - Sometimes, the library flagged by the SCA tool isn't insecure itself, but one it depends on might be.

- **Run-time protection tools**

  - Example: Web Application Firewalls (WAFs), also known as Application Security Managers (ASMs), offer run-time protection for HTTP applications by applying rules to HTTP conversations and analyzing web traffic.
  - They guard against common attacks like cross-site scripting (XSS) and SQL injection and can either report or block detected malicious patterns.
  - Most WAF vendors also offer defence against robotic attacks (bots), such as DDoS, and are enhancing their solutions with machine learning and AI for proactive protection.
  - Be careful;
    - These tools need to detect malicious activity to report or block it, implying that a vulnerability needs to be identified first.
    - There's a risk that these tools could block legitimate traffic, particularly in cases of high-volume activities like large batch jobs.
  - Be careful;
    - Proper management and vetting of the rules that govern WAFs and RASPs are crucial, as default rules may be too broad and trigger an overflow of alerts.
    - These rules should be well understood, tested, and jointly managed by the application security and development teams, as what may seem suspicious for one application might be legitimate for another.

- **Vulnerability collection and prioritisation**

  - Integrate defect tracking
  - Examples: Jira, Asana, Bugzilla, DefectDojo, Spreadsheet, Pen & Paper (if you brave enough)
  - The challenging part:
    - How do you get these vulnerabilities closed?
    - How do you prioritise the vulnerabilities?
    - Who owns the vulnerabilities?
    - What if the vulnerability cannot be closed?
    - Is there enough information for the development team to provide mitigation?
  - Prioritising vulnerabilities
    - Impact & Likelihood will give you the “Severity”
    - Time to closure
  - Closing vulnerabilities
    - Requires operational work
    - Needs a very close partnership with developers/engineers
    - What if you can't close vulnerability?
      - Find the stakeholders and;
        - Give them a clear picture
        - Ask for Risk management

- **Risk Management but how?**

  - **Risk Avoidance:** This approach mitigates risk by refraining from activities that could negatively impact the organisation.
  - **Risk Reduction:** This risk management method aims to limit the losses rather than completely eradicate them. It accepts the risk but works to contain potential losses and prevent their spread.
  - **Risk Sharing:** In this case, the risk of potential loss is distributed among a group instead of being borne by an individual.
  - **Transferring Risk:** This involves contractually shifting risk to a third party. For instance, insuring against property damage or injury transfers the associated risks from the property owner to the insurance company.
  - **Risk Acceptance and Retention:** After applying risk sharing, risk transfer, and risk reduction measures, some risk inevitably remains, as it's virtually impossible to eliminate all risks. This remaining risk is known as residual risk.

- **Vulnerability Disclosure Program**

  - Vulnerability Disclosure Programs (VDPs) encourage researchers to report security vulnerabilities without fearing retaliation.
  - VDPs facilitate the process of delivering security information to the appropriate team.
  - They address challenges faced by researchers, such as knowing where to send information and having an unclear resolution path post-submission.
  - VDPs aim to resolve these issues by implementing five core components of the policy.
    - Purpose of the policy
    - Scope
    - Safe Harbor
    - Process for submission
    - Expectations

- **Bug Bounty Program**

  - Bug Bounty Programs (BBPs) enhance VDPs by introducing incentives for submitted vulnerabilities.
  - The organisation sets predefined guidance for payout, either pricing specific vulnerabilities or varying costs by severity.
  - The organisation must plan for periodic bounty payouts.
  - BBPs can be private (only for invited researchers) or public (open to any researcher).
  - The organisation needs to triage submitted vulnerabilities, usually done by the application security team alongside the engineering team.
  - They need to confirm the vulnerability and agree on its severity.
  - Depending on the policy, it may allow public disclosure of the vulnerability if it isn't resolved within the stipulated time frame.
  - It's crucial for the organisation to follow their own policy in resolving the issue within set timeframes and to maintain communication with the researcher, to avoid a potentially public issue.

- **3. Shifting Left vs Shifting Right**

  - “Shift left" is the practice of integrating security early in the software development life cycle, ideally at the requirements gathering and development phase.
    - This term often emphasises the need to uncover security issues as early as possible in the development process.
    - **The ideal time to rectify a security vulnerability is at the moment of its creation.**
  - “Shift right” involves strategically placing tools throughout the development life cycle and production environment to identify and protect against vulnerabilities.
    - Penetration tests are conducted to discover potential issues.
    - The “shift right” model allows for verification of tool functionality and report generation to assess effectiveness.
    - This could disrupt the DevOps model, which emphasises speed and struggles with broken builds or gates.

- **3.1. Shifting Left**

  - Shifting left' prioritises early security efforts in the development lifecycle, being more cost-effective and efficient than resolving issues later.
  - Implementing 'shift left' can be challenging and might be overlooked if the organisation needs a quick solution.
  - Secure development starts with appropriate architecture and design choices, considering security aspects like session management, encryption, and authorization.
  - Utilising proven patterns and standards from reputable organisations, such as OWASP, aids in ensuring security.
  - Decisions around data protection schemes should coincide with architectural decisions on data flow and database technology.
  - Clear requirements for encryption strategies based on data classification should be established early.
  - Implementing field-level encryption with proper key lifecycle management is easier before massive data accumulation.
  - Challenges can arise with legacy applications, which may not support modern, granular encryption methods due to their design constraints. (Modern banking problem)

- **4. Known Attack Surfaces & Vectors**

- **Known Rust Vulnerabilities (Surface & Vectors)**

  - Rust-specific issues: Most common issues are related to misuse of some common features of the language. These include: panicking on error handling, arithmetic errors, resource exhaustion, stack overflow.
  - Unsafe code: Rust allows developers to write unsafe code blocks that bypass safety guarantees. It is absolutely needed in some cases, but such code may contain a number of vulnerabilities, including: use-after-free, out of bound access, double free, memory leaks and so on.
  - Cryptographic errors: There are a number of cryptographic libraries in the Rust ecosystem, but using these requires knowledge of the underlying algorithms, including their weaknesses and possible vulnerabilities.
  - API misuse: Incorrect usage of interfaces provided by third-party libraries may introduce vulnerabilities.

- **Known Substrate Vulnerabilities (Surface & Vectors)**

  - Insufficient testing: Being safety critical applications, blockchain projects must be rigorously tested, including following all the common practices of secure software lifecycle.
  - Centralization vulnerabilities: If centralized components are present in a substrate-based project, they can be a point of failure.
  - Pallet-specific vulnerabilities: Each pallet contains specific logic including its communication with other components of the project, which may introduce security issues.
  - Incorrect access control: A type of an error in the business logic of the smart contract that lets unauthorized users access privileged functions.

- **Known ink! Vulnerabilities (Surface & Vectors)**

  - Denial-of-Service (DoS) attacks: Execution of smart contracts requires some amount of gas. Therefore, presence of some implementation issues may lead to gas exhaustion in the contract making it non-functional.
  - Timestamp dependence: The developer always must be careful with all the operations that take into account the time in the system.
  - Outdated version of ink!: The latest version of ink! always should be used.

- **Resources used to create slides**

1.  Application Security Program Handbook, by Derek Fisher: <https://www.manning.com/books/application-security-program-handbook>
2.  Agile Application Security, by Laura Bell, Michael Brunton-Spall, Rich Smith & Jim Bird” <https://www.oreilly.com/library/view/agile-application-security/9781491938836/>
3.  CSSLP Certified Secure Software Lifecycle Professional All-in-One Exam Guide, Third Edition, by Wm. Arthur Conklin, Daniel Paul Shoemaker: <https://www.mheducation.co.uk/csslp-certified-secure-software-lifecycle-professional-all-in-one-exam-guide-third-edition-9781264258208-emea> <!-- markdown-link-check-disable-line -->

4.  (ISC)2 CISSP Certified Information Systems Security Professional Official Study Guide, by Mike Chapple, James Michael Stewart, Darril Gibson, David Seidl: <https://www.wiley.com/en-gb/(ISC)2+CISSP+Certified+Information+Systems+Security+Professional+Official+Study+Guide+&+Practice+Tests+Bundle,+3rd+Edition-p-9781119790020> <!-- markdown-link-check-disable-line -->
5.  CISSP for Dummies, Lawrence C. Miller, Peter H. Gregory: <https://www.wiley.com/en-gb/CISSP+For+Dummies,+7th+Edition-p-9781119806820>

6.  CISM Certified Information Security Manager Study Guide, by Mike Chapple: <https://www.wiley.com/en-gb/CISM+Certified+Information+Security+Manager+Study+Guide-p-9781119801948>
7.  Secure Rust Guidelines: <https://anssi-fr.github.io/rust-guide>
8.  OWASP Smart Contract Top 10: <https://owasp.org/www-project-smart-contract-top-10/>
9.  Smart Contract Weakness Classification and Test Cases: <https://swcregistry.io/>
