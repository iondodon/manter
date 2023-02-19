import type { DynamicGroup, Group, Suggestion } from "./contract";

const filesOrFolders: Suggestion = {
  names: "file or folder",
  regex: /^.*$/,
  next: () => [filesOrFolders],
};

const lsOptions: Group = {
  suggestions: [
    {
      names: "-a",
      regex: /^-a$/,
    },
    {
      names: "-l",
      regex: /^-l$/,
    },
  ],
  next: () => [lsOptions, filesOrFolders],
};

const cdFolders: DynamicGroup = {
  suggestions: [
    {
      names: "folder",
      regex: /^.*$/,
      next: () => [cdFolders],
    },
  ],
  script: "ls -d */",
  postProcessor: (line) => {
    return {
      names: line,
      regex: new RegExp("^" + line + "$"),
      next: () => [cdFolders],
    };
  },
};

const cdOptions: Group = {
  suggestions: [
    {
      names: ["-a", "--all"],
      regex: [/^-a$/, /^--all$/],
    },
    {
      names: ["-l", "--long"],
      regex: [/^-l$/, /^--long$/],
    },
  ],
  next: () => [cdOptions, cdFolders],
};

const branchNames: DynamicGroup = {
  suggestions: [
    {
      names: "branch",
      regex: /^.*$/,
    },
  ],
  script: "git for-each-ref --format='%(refname:short)' refs/heads/",
  postProcessor: (line) => {
    return {
      names: line,
      regex: new RegExp("^" + line + "$"),
    };
  },
};

const gitSubCommands: Group = {
  suggestions: [
    {
      names: "checkout",
      regex: /^checkout$/,
      next: () => [branchNames],
    },
  ],
};

const clis: Group = {
  suggestions: [
    {
      names: "ls",
      regex: /^ls$/,
      next: () => [lsOptions, filesOrFolders],
    },
    {
      names: "cd",
      regex: /^cd$/,
      next: () => [cdOptions, cdFolders],
    },
    {
      names: "git",
      regex: /^git$/,
      next: () => [gitSubCommands],
    },
  ],
};

export default clis;
