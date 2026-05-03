export type SkillResult<T> =
    | { ok: true; value: T }
    | { ok: false; error: string };

export function ok<T>(value: T): SkillResult<T> {
    return { ok: true, value };
}

export function err<T>(error: string): SkillResult<T> {
    return { ok: false, error };
}
