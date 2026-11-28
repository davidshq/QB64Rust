// The build settings of `qb64rust.build.*` as the `-f:` settings that both compilers take (`qb64rust` and
// `qb64pe.exe` read the same names: `qb64pe.bas`, its `-f:` options).

/** The values of `qb64rust.build.*` that become `-f:` settings. */
export interface CompilerSettings {
    optimize: boolean;
    licenseFile: boolean;
    extraLinkerFlags: string;
}

/** The `-f:` settings for the values, in a fixed order; a setting at its default is left out. */
export function compilerFlags(c: CompilerSettings): string[] {
    const out: string[] = [];
    if (c.optimize) {
        out.push("-f:OptimizeCppProgram=true");
    }
    if (c.licenseFile) {
        out.push("-f:GenerateLicenseFile=true");
    }
    if (c.extraLinkerFlags.trim()) {
        out.push(`-f:ExtraLinkerFlags=${c.extraLinkerFlags.trim()}`);
    }
    return out;
}
