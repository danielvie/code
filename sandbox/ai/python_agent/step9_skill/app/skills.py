from dataclasses import dataclass
from pathlib import Path

from openai.types.chat import ChatCompletionMessageParam

SKILLS_PATH = Path(".agents/skills")


@dataclass
class Skill:
    name: str
    description: str


@dataclass
class SkillContext:
    found: list[Skill]
    matched: list[Skill]
    messages: list[ChatCompletionMessageParam]

    @property
    def found_names(self) -> str:
        return ", ".join(skill.name for skill in self.found) or "none"

    @property
    def used_names(self) -> str:
        return ", ".join(skill.name for skill in self.matched) or "none"


def load_skills() -> list[Skill]:
    if not SKILLS_PATH.exists():
        return []

    skills: list[Skill] = []
    for skill_file in sorted(SKILLS_PATH.glob("*/SKILL.md")):
        text = skill_file.read_text(encoding="utf-8")
        name = skill_file.parent.name
        description = ""

        for line in text.splitlines():
            if line.startswith("name:"):
                name = line.removeprefix("name:").strip()
            if line.startswith("description:"):
                description = line.removeprefix("description:").strip()

        skills.append(Skill(name=name, description=description))

    return skills


def match_skills(prompt: str, skills: list[Skill]) -> list[Skill]:
    prompt_lower = prompt.lower()
    return [skill for skill in skills if skill.name.lower() in prompt_lower]


def build_skill_system_message(skills: list[Skill]) -> str:
    lines = [
        "The user prompt matched these local skills.",
        "Only the skill name and description are included here.",
        "If you need the full instructions for a matched skill, call LoadSkill with its name.",
        "",
        "Matched skills:",
    ]

    for skill in skills:
        lines.append(f"- {skill.name}: {skill.description}")

    return "\n".join(lines)


def build_skill_context(prompt: str) -> SkillContext:
    skills = load_skills()
    matched_skills = match_skills(prompt, skills)

    messages: list[ChatCompletionMessageParam] = []
    if matched_skills:
        messages.append(
            {"role": "system", "content": build_skill_system_message(matched_skills)}
        )

    return SkillContext(found=skills, matched=matched_skills, messages=messages)
