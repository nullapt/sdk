export * from './manifest';
export * from './types';

/**
 * Validates that a SKILL.json object matches the expected schema shape.
 * Throws a descriptive error if any required field is missing or malformed.
 */
export function validateManifest(raw: unknown): asserts raw is SkillManifest {
    const m = raw as Record<string, unknown>;
    const required = ['schema_version', 'name', 'version', 'description', 'author', 'entry', 'interface', 'permissions', 'signature'];
    for (const field of required) {
        if (!(field in m)) throw new Error(`SKILL.json missing required field: ${field}`);
    }
    const name = m['name'] as string;
    if (!/^[a-z0-9][a-z0-9_\-]*(\/[a-z0-9][a-z0-9_\-]*)?$/.test(name)) {
        throw new Error(`Invalid skill name: "${name}"`);
    }
}

import type { SkillManifest } from './manifest';
