// Auto-generated from ag-kit skills. Do not edit manually.
// Run: python3 scripts/generate_skills.py

// Embedded skill content accessors

#[allow(dead_code)]
const SKILL_API_PATTERNS: &str = include_str!("skills/api-patterns/SKILL.md");

#[allow(dead_code)]
const SKILL_APP_BUILDER: &str = include_str!("skills/app-builder/SKILL.md");

#[allow(dead_code)]
const SKILL_ARCHITECTURE: &str = include_str!("skills/architecture/SKILL.md");

#[allow(dead_code)]
const SKILL_BASH_LINUX: &str = include_str!("skills/bash-linux/SKILL.md");

#[allow(dead_code)]
const SKILL_BATCH_OPERATIONS: &str = include_str!("skills/batch-operations/SKILL.md");

#[allow(dead_code)]
const SKILL_BEHAVIORAL_MODES: &str = include_str!("skills/behavioral-modes/SKILL.md");

#[allow(dead_code)]
const SKILL_BRAINSTORMING: &str = include_str!("skills/brainstorming/SKILL.md");

#[allow(dead_code)]
const SKILL_CLEAN_CODE: &str = include_str!("skills/clean-code/SKILL.md");

#[allow(dead_code)]
const SKILL_CODE_REVIEW_CHECKLIST: &str = include_str!("skills/code-review-checklist/SKILL.md");

#[allow(dead_code)]
const SKILL_CODE_REVIEW_GRAPH: &str = include_str!("skills/code-review-graph/SKILL.md");

#[allow(dead_code)]
const SKILL_CONTEXT_COMPRESSION: &str = include_str!("skills/context-compression/SKILL.md");

#[allow(dead_code)]
const SKILL_COORDINATOR_MODE: &str = include_str!("skills/coordinator-mode/SKILL.md");

#[allow(dead_code)]
const SKILL_DATABASE_DESIGN: &str = include_str!("skills/database-design/SKILL.md");

#[allow(dead_code)]
const SKILL_DEPLOYMENT_PROCEDURES: &str = include_str!("skills/deployment-procedures/SKILL.md");

#[allow(dead_code)]
const SKILL_DESIGN_SPEC: &str = include_str!("skills/design-spec/SKILL.md");

#[allow(dead_code)]
const SKILL_DOCUMENTATION_TEMPLATES: &str = include_str!("skills/documentation-templates/SKILL.md");

#[allow(dead_code)]
const SKILL_FRONTEND_ARCHITECTURE: &str = include_str!("skills/frontend-architecture/SKILL.md");

#[allow(dead_code)]
const SKILL_FRONTEND_DESIGN: &str = include_str!("skills/frontend-design/SKILL.md");

#[allow(dead_code)]
const SKILL_GAME_DEVELOPMENT: &str = include_str!("skills/game-development/SKILL.md");

#[allow(dead_code)]
const SKILL_GEO_FUNDAMENTALS: &str = include_str!("skills/geo-fundamentals/SKILL.md");

#[allow(dead_code)]
const SKILL_I18N_LOCALIZATION: &str = include_str!("skills/i18n-localization/SKILL.md");

#[allow(dead_code)]
const SKILL_INTELLIGENT_ROUTING: &str = include_str!("skills/intelligent-routing/SKILL.md");

#[allow(dead_code)]
const SKILL_LINT_AND_VALIDATE: &str = include_str!("skills/lint-and-validate/SKILL.md");

#[allow(dead_code)]
const SKILL_MCP_BUILDER: &str = include_str!("skills/mcp-builder/SKILL.md");

#[allow(dead_code)]
const SKILL_MEMORY_SYSTEM: &str = include_str!("skills/memory-system/SKILL.md");

#[allow(dead_code)]
const SKILL_MOBILE_DESIGN: &str = include_str!("skills/mobile-design/SKILL.md");

#[allow(dead_code)]
const SKILL_NEXTJS_REACT_EXPERT: &str = include_str!("skills/nextjs-react-expert/SKILL.md");

#[allow(dead_code)]
const SKILL_NODEJS_BEST_PRACTICES: &str = include_str!("skills/nodejs-best-practices/SKILL.md");

#[allow(dead_code)]
const SKILL_PARALLEL_AGENTS: &str = include_str!("skills/parallel-agents/SKILL.md");

#[allow(dead_code)]
const SKILL_PERFORMANCE_PROFILING: &str = include_str!("skills/performance-profiling/SKILL.md");

#[allow(dead_code)]
const SKILL_PLAN_WRITING: &str = include_str!("skills/plan-writing/SKILL.md");

#[allow(dead_code)]
const SKILL_POWERSHELL_WINDOWS: &str = include_str!("skills/powershell-windows/SKILL.md");

#[allow(dead_code)]
const SKILL_PYTHON_PATTERNS: &str = include_str!("skills/python-patterns/SKILL.md");

#[allow(dead_code)]
const SKILL_RED_TEAM_TACTICS: &str = include_str!("skills/red-team-tactics/SKILL.md");

#[allow(dead_code)]
const SKILL_RUST_PRO: &str = include_str!("skills/rust-pro/SKILL.md");

#[allow(dead_code)]
const SKILL_SEO_FUNDAMENTALS: &str = include_str!("skills/seo-fundamentals/SKILL.md");

#[allow(dead_code)]
const SKILL_SERVER_MANAGEMENT: &str = include_str!("skills/server-management/SKILL.md");

#[allow(dead_code)]
const SKILL_SIMPLIFY_CODE: &str = include_str!("skills/simplify-code/SKILL.md");

#[allow(dead_code)]
const SKILL_SKILLIFY: &str = include_str!("skills/skillify/SKILL.md");

#[allow(dead_code)]
const SKILL_SYSTEMATIC_DEBUGGING: &str = include_str!("skills/systematic-debugging/SKILL.md");

#[allow(dead_code)]
const SKILL_TAILWIND_PATTERNS: &str = include_str!("skills/tailwind-patterns/SKILL.md");

#[allow(dead_code)]
const SKILL_TDD_WORKFLOW: &str = include_str!("skills/tdd-workflow/SKILL.md");

#[allow(dead_code)]
const SKILL_TESTING_PATTERNS: &str = include_str!("skills/testing-patterns/SKILL.md");

#[allow(dead_code)]
const SKILL_VERIFY_CHANGES: &str = include_str!("skills/verify-changes/SKILL.md");

#[allow(dead_code)]
const SKILL_VULNERABILITY_SCANNER: &str = include_str!("skills/vulnerability-scanner/SKILL.md");

#[allow(dead_code)]
const SKILL_WEB_DESIGN_GUIDELINES: &str = include_str!("skills/web-design-guidelines/SKILL.md");

#[allow(dead_code)]
const SKILL_WEBAPP_TESTING: &str = include_str!("skills/webapp-testing/SKILL.md");

// Reference file accessors

pub fn all_skills() -> Vec<Skill> {
    vec![
        Skill {
            name: "api-patterns".into(),
            description: "API design principles and decision-making. REST vs GraphQL vs tRPC selection, response formats, versioning, pagination.".into(),
            keywords: vec!["designing".into(), "rest".into(), "graphql".into(), "trpc".into(), "apis".into(), "defining".into(), "response".into(), "formats".into(), "versioning".into(), "pagination".into(), "api".into(), "authentication.".into(), "ui".into(), "frontend".into(), "work.".into()],
            when_to_use: "When designing REST/GraphQL/tRPC APIs, defining response formats, versioning, pagination, or API authentication. NOT for UI/frontend work.".into(),
            content: SKILL_API_PATTERNS.into(),
        },
        Skill {
            name: "app-builder".into(),
            description: "Main application building orchestrator. Creates full-stack applications from natural language requests. Determines project type, selects tech stack, coordinates agents.".into(),
            keywords: vec!["creating".into(), "new".into(), "full-stack".into(), "application".into(), "from".into(), "scratch".into(), "selecting".into(), "tech".into(), "stack".into(), "scaffolding".into(), "project".into(), "structure.".into(), "create".into(), "workflow.".into()],
            when_to_use: "When creating a new full-stack application from scratch, selecting tech stack, or scaffolding project structure. Use with /create workflow.".into(),
            content: SKILL_APP_BUILDER.into(),
        },
        Skill {
            name: "architecture".into(),
            description: "Architectural decision-making framework. Requirements analysis, trade-off evaluation, ADR documentation. Use when making architecture decisions or analyzing system design.".into(),
            keywords: vec!["making".into(), "architectural".into(), "decisions".into(), "evaluating".into(), "trade-offs".into(), "writing".into(), "adrs".into(), "analyzing".into(), "system".into(), "design.".into(), "direct".into(), "code".into(), "implementation.".into()],
            when_to_use: "When making architectural decisions, evaluating trade-offs, writing ADRs, or analyzing system design. NOT for direct code implementation.".into(),
            content: SKILL_ARCHITECTURE.into(),
        },
        Skill {
            name: "bash-linux".into(),
            description: "Bash/Linux terminal patterns. Critical commands, piping, error handling, scripting. Use when working on macOS or Linux systems.".into(),
            keywords: vec!["working".into(), "on".into(), "macos".into(), "linux".into(), "systems".into(), "writing".into(), "bash".into(), "scripts".into(), "using".into(), "terminal".into(), "commands.".into(), "windows".into(), "powershell".into(), "environments.".into()],
            when_to_use: "When working on macOS or Linux systems, writing bash scripts, or using terminal commands. NOT for Windows/PowerShell environments.".into(),
            content: SKILL_BASH_LINUX.into(),
        },
        Skill {
            name: "batch-operations".into(),
            description: "Apply operations across multiple files simultaneously. Pattern-based bulk modifications, search-and-replace across codebases, consistent changes to many files at once.".into(),
            keywords: vec!["user".into(), "needs".into(), "to".into(), "change".into(), "multiple".into(), "files".into(), "same".into(), "pattern".into(), "rename".into(), "across".into(), "codebase".into(), "add".into(), "imports".into(), "many".into(), "update".into()],
            when_to_use: "When the user needs to change multiple files with the same pattern, rename across a codebase, add imports to many files, update versions, or apply consistent modifications. NOT for single-file edits.".into(),
            content: SKILL_BATCH_OPERATIONS.into(),
        },
        Skill {
            name: "behavioral-modes".into(),
            description: "AI operational modes (brainstorm, implement, debug, review, teach, ship, orchestrate). Use to adapt behavior based on task type.".into(),
            keywords: vec!["adapting".into(), "ai".into(), "behavior".into(), "specific".into(), "task".into(), "types".into(), "brainstorm".into(), "implement".into(), "debug".into(), "review".into(), "teach".into(), "ship".into(), "orchestrate".into(), "modes.".into()],
            when_to_use: "When adapting AI behavior for specific task types: brainstorm, implement, debug, review, teach, ship, or orchestrate modes.".into(),
            content: SKILL_BEHAVIORAL_MODES.into(),
        },
        Skill {
            name: "brainstorming".into(),
            description: "Socratic questioning protocol + user communication. MANDATORY for complex requests, new features, or unclear requirements. Includes progress reporting and error handling.".into(),
            keywords: vec!["exploring".into(), "options".into(), "before".into(), "implementation".into(), "clarifying".into(), "requirements".into(), "user".into(), "needs".into(), "creative".into(), "problem-solving.".into(), "brainstorm".into(), "workflow.".into()],
            when_to_use: "When exploring options before implementation, clarifying requirements, or when the user needs creative problem-solving. Use with /brainstorm workflow.".into(),
            content: SKILL_BRAINSTORMING.into(),
        },
        Skill {
            name: "clean-code".into(),
            description: "Pragmatic coding standards - concise, direct, no over-engineering, no unnecessary comments".into(),
            keywords: vec!["always".into(), "active".into(), "all".into(), "code".into(), "writing.".into(), "enforces".into(), "concise".into(), "direct".into(), "coding".into(), "standards".into(), "testing".into(), "pyramid".into(), "performance".into(), "best".into(), "practices.".into()],
            when_to_use: "Always active for ALL code writing. Enforces concise, direct coding standards, testing pyramid, and performance best practices.".into(),
            content: SKILL_CLEAN_CODE.into(),
        },
        Skill {
            name: "code-review-checklist".into(),
            description: "Code review guidelines covering code quality, security, and best practices.".into(),
            keywords: vec!["reviewing".into(), "code".into(), "quality".into(), "security".into(), "best".into(), "practices.".into(), "user".into(), "says".into(), "review".into(), "my".into(), "check".into(), "pr".into()],
            when_to_use: "When reviewing code for quality, security, and best practices. When the user says 'review my code' or 'check this PR'.".into(),
            content: SKILL_CODE_REVIEW_CHECKLIST.into(),
        },
        Skill {
            name: "code-review-graph".into(),
            description: "Token-efficient code review using Tree-sitter AST graphs and MCP. Cuts AI token usage on large codebases by computing the blast radius of changes instead of reading entire codebases. Uses a SQLite graph database for structural analysis.".into(),
            keywords: vec!["reviewing".into(), "code".into(), "in".into(), "large".into(), "codebases".into(), "500+".into(), "files".into(), "token".into(), "costs".into(), "high".into(), "making".into(), "multi-file".into(), "changes".into(), "cross-module".into(), "dependencies".into()],
            when_to_use: "When reviewing code in large codebases (500+ files), when token costs are high, when making multi-file changes with cross-module dependencies, or when working with monorepos. Also for dead code detection, architecture visualization, and refactoring previews. NOT for small projects under 200 files with isolated single-file changes.".into(),
            content: SKILL_CODE_REVIEW_GRAPH.into(),
        },
        Skill {
            name: "context-compression".into(),
            description: "Manage and compress conversation context in long sessions. Detect when context is growing large, summarize completed work phases, archive old findings while preserving key decisions. Prevents context degradation.".into(),
            keywords: vec!["session".into(), "has".into(), "20+".into(), "turns".into(), "context".into(), "feels".into(), "repetitive".into(), "agent".into(), "is".into(), "losing".into(), "track".into(), "of".into(), "earlier".into(), "work".into(), "user".into()],
            when_to_use: "When a session has 20+ turns, when context feels repetitive, when the agent is losing track of earlier work, or when the user says 'summarize what we've done'. NOT for short sessions.".into(),
            content: SKILL_CONTEXT_COMPRESSION.into(),
        },
        Skill {
            name: "coordinator-mode".into(),
            description: "Advanced multi-agent orchestration with parallel workers, synthesis protocols, and coordinator lifecycle. Use when complex tasks require multiple agents working in parallel with intelligent result synthesis.".into(),
            keywords: vec!["user".into(), "needs".into(), "multi-agent".into(), "coordination".into(), "parallel".into(), "task".into(), "execution".into(), "complex".into(), "multi-domain".into(), "work".into(), "coordinate".into(), "orchestrate".into(), "is".into(), "invoked.".into(), "single-domain".into()],
            when_to_use: "When the user needs multi-agent coordination, parallel task execution, complex multi-domain work, or when /coordinate or /orchestrate is invoked. NOT for single-domain tasks.".into(),
            content: SKILL_COORDINATOR_MODE.into(),
        },
        Skill {
            name: "database-design".into(),
            description: "Database design principles and decision-making. Schema design, indexing strategy, ORM selection, serverless databases.".into(),
            keywords: vec!["designing".into(), "database".into(), "schemas".into(), "choosing".into(), "orms".into(), "planning".into(), "migrations".into(), "optimizing".into(), "queries.".into(), "working".into(), "prisma".into(), "drizzle".into(), "sql".into(), "files.".into()],
            when_to_use: "When designing database schemas, choosing ORMs, planning migrations, or optimizing queries. When working with Prisma, Drizzle, or SQL files.".into(),
            content: SKILL_DATABASE_DESIGN.into(),
        },
        Skill {
            name: "deployment-procedures".into(),
            description: "Production deployment principles and decision-making. Safe deployment workflows, rollback strategies, and verification. Teaches thinking, not scripts.".into(),
            keywords: vec!["deploying".into(), "to".into(), "production".into(), "planning".into(), "rollback".into(), "strategies".into(), "setting".into(), "up".into(), "ci".into(), "cd".into(), "pipelines.".into(), "deploy".into(), "workflow.".into()],
            when_to_use: "When deploying to production, planning rollback strategies, or setting up CI/CD pipelines. Use with /deploy workflow.".into(),
            content: SKILL_DEPLOYMENT_PROCEDURES.into(),
        },
        Skill {
            name: "design-spec".into(),
            description: "How to author a DESIGN.md file — the machine-readable design-token + human-rationale format that must exist before any UI is built. YAML front-matter token schema (colors, typography, spacing, rounded, components), type system, token references, and canonical section order.".into(),
            keywords: vec!["before".into(), "writing".into(), "any".into(), "ui".into(), "code".into(), "web".into(), "mobile".into(), "read".into(), "creating".into(), "updating".into(), "project".into(), "design.md".into(), "defining".into(), "design".into(), "tokens".into()],
            when_to_use: "BEFORE writing any UI code (web or mobile). Read when creating or updating a project's DESIGN.md, defining design tokens, or when a UI task needs a design source-of-truth. Pair with frontend-design (web aesthetics) or mobile-design (mobile).".into(),
            content: SKILL_DESIGN_SPEC.into(),
        },
        Skill {
            name: "documentation-templates".into(),
            description: "Documentation templates and structure guidelines. README, API docs, code comments, and AI-friendly documentation.".into(),
            keywords: vec!["writing".into(), "readme".into(), "files".into(), "api".into(), "documentation".into(), "code".into(), "comments".into(), "generating".into(), "ai-friendly".into(), "documentation.".into()],
            when_to_use: "When writing README files, API documentation, code comments, or generating AI-friendly documentation.".into(),
            content: SKILL_DOCUMENTATION_TEMPLATES.into(),
        },
        Skill {
            name: "frontend-architecture".into(),
            description: "How to organize frontend code — separation of concerns (UI / logic / data / type), file responsibility, state tiers, API services, schema validation, and framework conventions for React/Next and Vue. Structural rules, not visual design.".into(),
            keywords: vec!["structuring".into(), "frontend".into(), "codebase".into(), "reviewing".into(), "how".into(), "code".into(), "is".into(), "organized".into(), "where".into(), "logic".into(), "api".into(), "calls".into(), "state".into(), "types".into(), "validation".into()],
            when_to_use: "When structuring a frontend codebase or reviewing how frontend code is organized — where logic, API calls, state, types, and validation should live; component vs hook/composable boundaries; Next.js server/client split; Vue Composition API. NOT for visual design (use frontend-design) and NOT for React/Next performance rules (use nextjs-react-expert).".into(),
            content: SKILL_FRONTEND_ARCHITECTURE.into(),
        },
        Skill {
            name: "frontend-design".into(),
            description: "Anti-slop frontend design for web UI — landing pages, portfolios, marketing/product sites, and redesigns. Reads the brief, infers the right direction, and ships interfaces that don't look templated. Real design systems when applicable, audit-first on redesigns, strict pre-flight check. NOT for mobile apps.".into(),
            keywords: vec!["designing".into(), "building".into(), "web".into(), "ui".into(), "components".into(), "layouts".into(), "color".into(), "typography".into(), "landing".into(), "pages".into(), "redesigns.".into(), "mobile".into(), "apps".into(), "mobile-design".into()],
            when_to_use: "When designing or building web UI — components, layouts, color, typography, landing pages, or redesigns. NOT for mobile apps (use mobile-design).".into(),
            content: SKILL_FRONTEND_DESIGN.into(),
        },
        Skill {
            name: "game-development".into(),
            description: "Game development orchestrator. Routes to platform-specific skills based on project needs.".into(),
            keywords: vec!["building".into(), "games".into(), "unity".into(), "godot".into(), "unreal".into(), "phaser".into(), "any".into(), "game".into(), "engine.".into(), "routes".into(), "to".into(), "platform-specific".into(), "sub-skills.".into()],
            when_to_use: "When building games with Unity, Godot, Unreal, Phaser, or any game engine. Routes to platform-specific sub-skills.".into(),
            content: SKILL_GAME_DEVELOPMENT.into(),
        },
        Skill {
            name: "geo-fundamentals".into(),
            description: "Generative Engine Optimization for AI search engines (ChatGPT, Claude, Perplexity).".into(),
            keywords: vec!["optimizing".into(), "content".into(), "ai".into(), "search".into(), "engines".into(), "like".into(), "chatgpt".into(), "claude".into(), "perplexity.".into(), "generative".into(), "engine".into(), "optimization.".into()],
            when_to_use: "When optimizing content for AI search engines like ChatGPT, Claude, or Perplexity. Generative Engine Optimization.".into(),
            content: SKILL_GEO_FUNDAMENTALS.into(),
        },
        Skill {
            name: "i18n-localization".into(),
            description: "Internationalization and localization patterns. Detecting hardcoded strings, managing translations, locale files, RTL support.".into(),
            keywords: vec!["internationalizing".into(), "an".into(), "app".into(), "managing".into(), "translations".into(), "detecting".into(), "hardcoded".into(), "strings".into(), "adding".into(), "rtl".into(), "support.".into()],
            when_to_use: "When internationalizing an app, managing translations, detecting hardcoded strings, or adding RTL support.".into(),
            content: SKILL_I18N_LOCALIZATION.into(),
        },
        Skill {
            name: "intelligent-routing".into(),
            description: "Automatic agent selection and intelligent task routing. Analyzes user requests and automatically selects the best specialist agent(s) without requiring explicit user mentions.".into(),
            keywords: vec!["always".into(), "active.".into(), "automatically".into(), "selects".into(), "best".into(), "specialist".into(), "agent".into(), "each".into(), "user".into(), "request".into(), "without".into(), "explicit".into(), "mentions.".into()],
            when_to_use: "Always active. Automatically selects the best specialist agent for each user request without explicit user mentions.".into(),
            content: SKILL_INTELLIGENT_ROUTING.into(),
        },
        Skill {
            name: "lint-and-validate".into(),
            description: "Automatic quality control, linting, and static analysis procedures. Use after every code modification to ensure syntax correctness and project standards. Triggers on keywords: lint, format, check, validate, types, static analysis.".into(),
            keywords: vec!["running".into(), "linters".into(), "type".into(), "checkers".into(), "code".into(), "formatters.".into(), "after".into(), "any".into(), "change".into(), "that".into(), "needs".into(), "quality".into(), "validation.".into()],
            when_to_use: "When running linters, type checkers, or code formatters. After any code change that needs quality validation.".into(),
            content: SKILL_LINT_AND_VALIDATE.into(),
        },
        Skill {
            name: "mcp-builder".into(),
            description: "MCP (Model Context Protocol) server building principles. Tool design, resource patterns, best practices.".into(),
            keywords: vec!["building".into(), "mcp".into(), "model".into(), "context".into(), "protocol".into(), "servers".into(), "designing".into(), "tools".into(), "implementing".into(), "resource".into(), "patterns.".into()],
            when_to_use: "When building MCP (Model Context Protocol) servers, designing MCP tools, or implementing MCP resource patterns.".into(),
            content: SKILL_MCP_BUILDER.into(),
        },
        Skill {
            name: "memory-system".into(),
            description: "Persistent cross-session memory management. Enables agents to remember user preferences, project conventions, and past decisions across different sessions using a structured MEMORY.md index and topic files.".into(),
            keywords: vec!["user".into(), "says".into(), "remember".into(), "save".into(), "later".into(), "don".into(), "forget".into(), "starting".into(), "new".into(), "session".into(), "needing".into(), "to".into(), "recall".into(), "past".into(), "context.".into()],
            when_to_use: "When the user says 'remember this', 'save this for later', 'don't forget', or when starting a new session and needing to recall past context. Also when /remember workflow is invoked.".into(),
            content: SKILL_MEMORY_SYSTEM.into(),
        },
        Skill {
            name: "mobile-design".into(),
            description: "Mobile-first design thinking and decision-making for iOS and Android apps. Touch interaction, performance patterns, platform conventions. Teaches principles, not fixed values. Use when building React Native, Flutter, or native mobile apps.".into(),
            keywords: vec!["designing".into(), "mobile".into(), "app".into(), "interfaces".into(), "ios".into(), "android".into(), "react".into(), "native".into(), "flutter.".into(), "touch".into(), "interaction".into(), "platform".into(), "conventions.".into(), "web".into(), "apps.".into()],
            when_to_use: "When designing mobile app interfaces for iOS/Android, React Native, or Flutter. Touch interaction and platform conventions. NOT for web apps.".into(),
            content: SKILL_MOBILE_DESIGN.into(),
        },
        Skill {
            name: "nextjs-react-expert".into(),
            description: "React and Next.js performance optimization from Vercel Engineering. Use when building React components, optimizing performance, eliminating waterfalls, reducing bundle size, reviewing code for performance issues, or implementing server/client-side optimizations.".into(),
            keywords: vec!["building".into(), "react".into(), "components".into(), "optimizing".into(), "next.js".into(), "performance".into(), "eliminating".into(), "waterfalls".into(), "reducing".into(), "bundle".into(), "size.".into(), "web".into(), "projects.".into()],
            when_to_use: "When building React components, optimizing Next.js performance, eliminating waterfalls, or reducing bundle size. For React/Next.js web projects.".into(),
            content: SKILL_NEXTJS_REACT_EXPERT.into(),
        },
        Skill {
            name: "nodejs-best-practices".into(),
            description: "Node.js development principles and decision-making. Framework selection, async patterns, security, and architecture. Teaches thinking, not copying.".into(),
            keywords: vec!["building".into(), "node.js".into(), "backends".into(), "selecting".into(), "frameworks".into(), "express".into(), "fastify".into(), "nestjs".into(), "implementing".into(), "async".into(), "patterns.".into()],
            when_to_use: "When building Node.js backends, selecting frameworks (Express/Fastify/NestJS), or implementing async patterns.".into(),
            content: SKILL_NODEJS_BEST_PRACTICES.into(),
        },
        Skill {
            name: "parallel-agents".into(),
            description: "Multi-agent orchestration patterns. Use when multiple independent tasks can run with different domain expertise or when comprehensive analysis requires multiple perspectives.".into(),
            keywords: vec!["task".into(), "requires".into(), "2+".into(), "specialist".into(), "agents".into(), "comprehensive".into(), "multi-domain".into(), "analysis".into(), "coordinated".into(), "parallel".into(), "execution.".into(), "orchestrate".into(), "coordinate".into(), "workflows.".into(), "single-domain".into()],
            when_to_use: "When a task requires 2+ specialist agents, comprehensive multi-domain analysis, or coordinated parallel execution. Use with /orchestrate or /coordinate workflows. NOT for single-domain tasks where one agent suffices.".into(),
            content: SKILL_PARALLEL_AGENTS.into(),
        },
        Skill {
            name: "performance-profiling".into(),
            description: "Performance profiling principles. Measurement, analysis, and optimization techniques.".into(),
            keywords: vec!["diagnosing".into(), "performance".into(), "issues".into(), "running".into(), "lighthouse".into(), "audits".into(), "analyzing".into(), "bundle".into(), "size".into(), "optimizing".into(), "core".into(), "web".into(), "vitals.".into()],
            when_to_use: "When diagnosing performance issues, running Lighthouse audits, analyzing bundle size, or optimizing Core Web Vitals.".into(),
            content: SKILL_PERFORMANCE_PROFILING.into(),
        },
        Skill {
            name: "plan-writing".into(),
            description: "Structured task planning with clear breakdowns, dependencies, and verification criteria. Use when implementing features, refactoring, or any multi-step work.".into(),
            keywords: vec!["creating".into(), "structured".into(), "task".into(), "plans".into(), "breaking".into(), "down".into(), "features".into(), "into".into(), "tasks".into(), "defining".into(), "verification".into(), "criteria.".into(), "plan".into(), "workflow.".into()],
            when_to_use: "When creating structured task plans, breaking down features into tasks, or defining verification criteria. Use with /plan workflow.".into(),
            content: SKILL_PLAN_WRITING.into(),
        },
        Skill {
            name: "powershell-windows".into(),
            description: "PowerShell Windows patterns. Critical pitfalls, operator syntax, error handling.".into(),
            keywords: vec!["working".into(), "on".into(), "windows".into(), "systems".into(), "writing".into(), "powershell".into(), "scripts".into(), "using".into(), "windows-specific".into(), "commands.".into(), "macos".into(), "linux.".into()],
            when_to_use: "When working on Windows systems, writing PowerShell scripts, or using Windows-specific commands. NOT for macOS/Linux.".into(),
            content: SKILL_POWERSHELL_WINDOWS.into(),
        },
        Skill {
            name: "python-patterns".into(),
            description: "Python development principles and decision-making. Framework selection, async patterns, type hints, project structure. Teaches thinking, not copying.".into(),
            keywords: vec!["writing".into(), "python".into(), "code".into(), "selecting".into(), "frameworks".into(), "implementing".into(), "type".into(), "hints".into(), "structuring".into(), "projects.".into()],
            when_to_use: "When writing Python code, selecting Python frameworks, implementing type hints, or structuring Python projects.".into(),
            content: SKILL_PYTHON_PATTERNS.into(),
        },
        Skill {
            name: "red-team-tactics".into(),
            description: "Red team tactics principles based on MITRE ATT&CK. Attack phases, detection evasion, reporting.".into(),
            keywords: vec!["performing".into(), "penetration".into(), "testing".into(), "red".into(), "team".into(), "exercises".into(), "evaluating".into(), "attack".into(), "surfaces".into(), "using".into(), "mitre".into(), "att".into(), "ck".into(), "framework.".into()],
            when_to_use: "When performing penetration testing, red team exercises, or evaluating attack surfaces using MITRE ATT&CK framework.".into(),
            content: SKILL_RED_TEAM_TACTICS.into(),
        },
        Skill {
            name: "rust-pro".into(),
            description: "Master modern Rust (2024 edition) with async patterns, advanced type system features, and production-ready systems programming. Expert in the current Rust ecosystem including Tokio, axum, and modern crates. Use PROACTIVELY for Rust development, performance optimization, or systems programming.".into(),
            keywords: vec!["writing".into(), "rust".into(), "code".into(), "working".into(), ".rs".into(), "files".into(), "cargo.toml".into(), "tokio".into(), "axum".into(), "any".into(), "ecosystem".into(), "tools.".into()],
            when_to_use: "When writing Rust code, working with .rs files, Cargo.toml, Tokio, axum, or any Rust ecosystem tools.".into(),
            content: SKILL_RUST_PRO.into(),
        },
        Skill {
            name: "seo-fundamentals".into(),
            description: "SEO fundamentals, E-E-A-T, Core Web Vitals, and Google algorithm principles.".into(),
            keywords: vec!["optimizing".into(), "web".into(), "pages".into(), "search".into(), "engines".into(), "implementing".into(), "meta".into(), "tags".into(), "improving".into(), "e-e-a-t".into(), "fixing".into(), "core".into(), "vitals.".into()],
            when_to_use: "When optimizing web pages for search engines, implementing meta tags, improving E-E-A-T, or fixing Core Web Vitals.".into(),
            content: SKILL_SEO_FUNDAMENTALS.into(),
        },
        Skill {
            name: "server-management".into(),
            description: "Server management principles and decision-making. Process management, monitoring strategy, and scaling decisions. Teaches thinking, not commands.".into(),
            keywords: vec!["managing".into(), "servers".into(), "configuring".into(), "process".into(), "managers".into(), "pm2".into(), "setting".into(), "up".into(), "monitoring".into(), "planning".into(), "scaling".into(), "strategies.".into()],
            when_to_use: "When managing servers, configuring process managers (PM2), setting up monitoring, or planning scaling strategies.".into(),
            content: SKILL_SERVER_MANAGEMENT.into(),
        },
        Skill {
            name: "simplify-code".into(),
            description: "Reduce complexity of over-engineered code. Identify unnecessary abstractions, remove dead code, flatten deep nesting, and simplify logic while preserving behavior.".into(),
            keywords: vec!["code".into(), "is".into(), "over-engineered".into(), "overly".into(), "abstract".into(), "deeply".into(), "nested".into(), "more".into(), "complex".into(), "than".into(), "needed.".into(), "user".into(), "asks".into(), "to".into(), "simplify".into()],
            when_to_use: "When code is over-engineered, overly abstract, deeply nested, or more complex than needed. When user asks to 'simplify', 'clean up', 'reduce complexity', or 'make this simpler'. NOT for adding new features.".into(),
            content: SKILL_SIMPLIFY_CODE.into(),
        },
        Skill {
            name: "skillify".into(),
            description: "Auto-create new skills from repetitive workflows. When you notice yourself doing the same multi-step process repeatedly, extract it into a reusable SKILL.md that any agent can use.".into(),
            keywords: vec!["user".into(), "says".into(), "make".into(), "skill".into(), "create".into(), "keep".into(), "doing".into(), "same".into(), "thing".into(), "repetitive".into(), "multi-step".into(), "pattern".into(), "is".into(), "observed.".into(), "one-off".into()],
            when_to_use: "When the user says 'make this a skill', 'create a skill for this', 'I keep doing this same thing', or when a repetitive multi-step pattern is observed. NOT for one-off tasks.".into(),
            content: SKILL_SKILLIFY.into(),
        },
        Skill {
            name: "systematic-debugging".into(),
            description: "4-phase systematic debugging methodology with root cause analysis and evidence-based verification. Use when debugging complex issues.".into(),
            keywords: vec!["debugging".into(), "complex".into(), "issues".into(), "performing".into(), "root".into(), "cause".into(), "analysis".into(), "using".into(), "evidence-based".into(), "problem".into(), "solving.".into(), "debug".into(), "workflow.".into()],
            when_to_use: "When debugging complex issues, performing root cause analysis, or using evidence-based problem solving. Use with /debug workflow.".into(),
            content: SKILL_SYSTEMATIC_DEBUGGING.into(),
        },
        Skill {
            name: "tailwind-patterns".into(),
            description: "Tailwind CSS v4 principles. CSS-first configuration, container queries, modern patterns, design token architecture.".into(),
            keywords: vec!["using".into(), "tailwind".into(), "css".into(), "v4".into(), "implementing".into(), "design".into(), "tokens".into(), "container".into(), "queries".into(), "modern".into(), "patterns".into(), "tailwind.".into()],
            when_to_use: "When using Tailwind CSS v4, implementing design tokens, container queries, or modern CSS patterns with Tailwind.".into(),
            content: SKILL_TAILWIND_PATTERNS.into(),
        },
        Skill {
            name: "tdd-workflow".into(),
            description: "Test-Driven Development workflow principles. RED-GREEN-REFACTOR cycle.".into(),
            keywords: vec!["practicing".into(), "test-driven".into(), "development".into(), "following".into(), "red-green-refactor".into(), "cycle".into(), "writing".into(), "tests".into(), "before".into(), "implementation.".into()],
            when_to_use: "When practicing Test-Driven Development, following RED-GREEN-REFACTOR cycle, or writing tests before implementation.".into(),
            content: SKILL_TDD_WORKFLOW.into(),
        },
        Skill {
            name: "testing-patterns".into(),
            description: "Testing patterns and principles. Unit, integration, mocking strategies.".into(),
            keywords: vec!["writing".into(), "unit".into(), "tests".into(), "integration".into(), "choosing".into(), "testing".into(), "frameworks".into(), "implementing".into(), "mocking".into(), "strategies.".into()],
            when_to_use: "When writing unit tests, integration tests, choosing testing frameworks, or implementing mocking strategies.".into(),
            content: SKILL_TESTING_PATTERNS.into(),
        },
        Skill {
            name: "verify-changes".into(),
            description: "Prove code works by running it, not just checking it exists. Verification through execution rather than inspection. Use after writing or modifying code to ensure it actually functions correctly.".into(),
            keywords: vec!["after".into(), "writing".into(), "code".into(), "completing".into(), "feature".into(), "fixing".into(), "bug.".into(), "user".into(), "says".into(), "does".into(), "work".into(), "test".into(), "verify".into(), "workflow".into(), "is".into()],
            when_to_use: "After writing code, completing a feature, or fixing a bug. When the user says 'does this work?', 'test this', 'verify', or when /verify workflow is invoked. NOT for writing new code — for proving existing code works.".into(),
            content: SKILL_VERIFY_CHANGES.into(),
        },
        Skill {
            name: "vulnerability-scanner".into(),
            description: "Advanced vulnerability analysis principles. OWASP 2025, Supply Chain Security, attack surface mapping, risk prioritization.".into(),
            keywords: vec!["scanning".into(), "security".into(), "vulnerabilities".into(), "checking".into(), "owasp".into(), "2025".into(), "compliance".into(), "analyzing".into(), "supply".into(), "chain".into(), "security.".into()],
            when_to_use: "When scanning for security vulnerabilities, checking OWASP 2025 compliance, or analyzing supply chain security.".into(),
            content: SKILL_VULNERABILITY_SCANNER.into(),
        },
        Skill {
            name: "web-design-guidelines".into(),
            description: "Review UI code for Web Interface Guidelines compliance. Use when asked to \"review my UI\", \"check accessibility\", \"audit design\", \"review UX\", or \"check my site against best practices\".".into(),
            keywords: vec!["auditing".into(), "web".into(), "ui".into(), "best".into(), "practices".into(), "checking".into(), "accessibility".into(), "reviewing".into(), "design".into(), "against".into(), "interface".into(), "guidelines.".into()],
            when_to_use: "When auditing web UI for best practices, checking accessibility, or reviewing design against Web Interface Guidelines.".into(),
            content: SKILL_WEB_DESIGN_GUIDELINES.into(),
        },
        Skill {
            name: "webapp-testing".into(),
            description: "Web application testing principles. E2E, Playwright, deep audit strategies.".into(),
            keywords: vec!["writing".into(), "e2e".into(), "tests".into(), "playwright".into(), "performing".into(), "deep".into(), "web".into(), "app".into(), "audits".into(), "testing".into(), "user".into(), "flows.".into(), "test".into(), "workflow.".into()],
            when_to_use: "When writing E2E tests with Playwright, performing deep web app audits, or testing user flows. Use with /test workflow.".into(),
            content: SKILL_WEBAPP_TESTING.into(),
        },
    ]
}
