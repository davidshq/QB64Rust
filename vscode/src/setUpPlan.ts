// The questions of "QB64: Set Up Project" from the plan `qb64rust init --plan` gives (change project-init, design
// D8; spec editor/project-setup). Pure, so that the unit tests cover it; setUpProject.ts shows them.
import type { InitAnswers, InitPlan } from "./compiler/qb64rust";

/** One choice of a pick list: what it shows and the answer it gives. */
export interface Choice {
    label: string;
    description?: string;
    value: string;
}

/** One pick list. With `many`, every choice is ticked at the start. */
export interface Question {
    key: "main" | "output" | "copy";
    title: string;
    choices: Choice[];
    many: boolean;
}

/** The output folder the command offers beside the source's own. */
export const BUILD_FOLDER = "build";

/** What the command does with a folder before any question (spec "Folders the command does not set up"). */
export type Start = { kind: "open"; settings: string } | { kind: "newProject" } | { kind: "ask"; questions: Question[] };

/** The questions before the data files: the main file when there are several, and where the program goes. */
export function startOf(plan: InitPlan): Start {
    if (plan.settings) {
        return { kind: "open", settings: plan.settings };
    }
    if (plan.newProject) {
        return { kind: "newProject" };
    }
    const questions: Question[] = [];
    if (plan.programs.length > 1) {
        questions.push({
            key: "main",
            title: "QB64: which file is the program?",
            choices: plan.programs.map((p) => ({ label: p, value: p })),
            many: false,
        });
    }
    questions.push({
        key: "output",
        title: "QB64: where does the program go?",
        choices: [
            { label: "Beside the source", description: "as QB64pe puts it", value: "." },
            { label: `In a folder \`${BUILD_FOLDER}\``, description: "with the data files it reads copied there", value: BUILD_FOLDER },
        ],
        many: false,
    });
    return { kind: "ask", questions };
}

/** The data files question, asked after the output folder `build` was chosen, when the folder has data files. */
export function copyQuestion(plan: InitPlan, output: string): Question | undefined {
    if (output === "." || plan.dataFiles.length === 0) {
        return undefined;
    }
    return {
        key: "copy",
        title: `QB64: which files does the program read? They are copied into \`${output}\` before each build.`,
        choices: plan.dataFiles.map((f) => ({ label: f, value: f })),
        many: true,
    };
}

/** The answers from the values picked, by question. */
export function answersOf(picked: Partial<Record<Question["key"], string[]>>): InitAnswers {
    const answers: InitAnswers = { output: picked.output?.[0] ?? ".", copy: picked.copy ?? [] };
    if (picked.main?.[0]) {
        answers.main = picked.main[0];
    }
    return answers;
}
