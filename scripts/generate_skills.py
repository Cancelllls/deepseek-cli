#!/usr/bin/env python3
"""Generate skills_data.rs from the ag-kit skills directory."""
import os
import re

SKILLS_DIR = "src/skills"
OUTPUT = "src/skills_data.rs"

def parse_frontmatter(content):
    """Extract YAML frontmatter from a markdown file."""
    if not content.startswith("---"):
        return {}, content

    parts = content.split("---", 2)
    if len(parts) < 3:
        return {}, content

    fm = parts[1].strip()
    body = parts[2].strip()

    meta = {}
    for line in fm.split("\n"):
        if ":" in line:
            key, _, val = line.partition(":")
            key = key.strip()
            val = val.strip().strip('"').strip("'")
            meta[key] = val

    return meta, body


def main():
    skill_dirs = sorted(os.listdir(SKILLS_DIR))
    skills = []

    for d in skill_dirs:
        skill_path = os.path.join(SKILLS_DIR, d)
        if not os.path.isdir(skill_path):
            continue

        md_path = os.path.join(skill_path, "SKILL.md")
        if not os.path.exists(md_path):
            continue

        with open(md_path) as f:
            content = f.read()

        meta, body = parse_frontmatter(content)
        name = meta.get("name", d)
        description = meta.get("description", "")
        when_to_use = meta.get("when_to_use", "")

        # Extract keywords from when_to_use
        keywords = extract_keywords(when_to_use)

        # Bundle reference files if they exist
        refs = []
        refs_dir = os.path.join(skill_path, "references")
        if os.path.isdir(refs_dir):
            for ref_file in sorted(os.listdir(refs_dir)):
                if ref_file.endswith(".md"):
                    refs.append(ref_file)

        skills.append({
            "dir": d,
            "name": name,
            "description": description,
            "when_to_use": when_to_use,
            "keywords": keywords,
            "has_body": bool(body),
            "refs": refs,
        })

    # Generate Rust source
    lines = []
    lines.append("// Auto-generated from ag-kit skills. Do not edit manually.")
    lines.append("// Run: python3 scripts/generate_skills.py")
    lines.append("")

    lines.append("// Embedded skill content accessors")
    for s in skills:
        safe_dir = s["dir"].replace("-", "_")
        path = f"skills/{s['dir']}/SKILL.md"
        lines.append(f"")
        lines.append(f"#[allow(dead_code)]")
        lines.append(f"const SKILL_{safe_dir.upper()}: &str = include_str!(\"{path}\");")

    lines.append("")
    lines.append("// Reference file accessors")
    for s in skills:
        for ref_file in s["refs"]:
            safe_dir = s["dir"].replace("-", "_")
            safe_ref = ref_file.replace("-", "_").replace(".", "_")
            path = f"skills/{s['dir']}/references/{ref_file}"
            lines.append(f"#[allow(dead_code)]")
            lines.append(f"const REF_{safe_dir.upper()}_{safe_ref.upper()}: &str = include_str!(\"{path}\");")

    lines.append("")
    lines.append("pub fn all_skills() -> Vec<Skill> {")
    lines.append("    vec![")
    for s in skills:
        safe_dir = s["dir"].replace("-", "_")
        kw_items = [f'"{k}".into()' for k in s["keywords"][:15]]
        kw_str = ", ".join(kw_items)
        desc = s["description"].replace('"', '\\"')
        wtu = s["when_to_use"].replace('"', '\\"')
        lines.append(f"        Skill {{")
        lines.append(f'            name: "{s["name"]}".into(),')
        lines.append(f'            description: "{desc}".into(),')
        lines.append(f"            keywords: vec![{kw_str}],")
        lines.append(f'            when_to_use: "{wtu}".into(),')
        lines.append(f"            content: SKILL_{safe_dir.upper()}.into(),")
        lines.append(f"        }},")
    lines.append("    ]")
    lines.append("}")

    with open(OUTPUT, "w") as f:
        f.write("\n".join(lines) + "\n")

    print(f"Generated {OUTPUT} with {len(skills)} skills")


def extract_keywords(when_to_use):
    """Extract searchable keywords from the when_to_use string."""
    if not when_to_use:
        return []

    text = when_to_use.lower()
    keywords = []

    # Extract quoted or mentioned technologies
    techs = re.findall(r'[\w.#+-]+', text)
    for t in techs:
        t = t.strip()
        if len(t) >= 2 and not t.startswith("when") and t not in ("and", "the", "or", "not", "for", "with", "use", "this", "you", "are"):
            keywords.append(t)

    return list(dict.fromkeys(keywords))  # dedupe


if __name__ == "__main__":
    os.makedirs("scripts", exist_ok=True)
    main()
