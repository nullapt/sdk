export interface SkillManifest {
    schema_version: string;
    name: string;
    version: string;
    description: string;
    author: string;
    homepage?: string;
    license: string;
    permissions: Permissions;
    entry: string;
    interface: SkillInterface;
    signature: Signature;
}

export interface Permissions {
    network: NetworkPerms;
    filesystem: FsPerms;
    env: string[];
}

export interface NetworkPerms {
    allowed: boolean;
    domains: string[];
}

export interface FsPerms {
    read: string[];
    write: string[];
}

export interface SkillInterface {
    tools: Tool[];
}

export interface Tool {
    name: string;
    description: string;
    input_schema: JSONSchema;
}

export type JSONSchema = Record<string, unknown>;
