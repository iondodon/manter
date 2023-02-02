import type { DynamicGroup, Group, Suggestion } from "./contract";

const filesOrFolders: Suggestion = {
  name: "file or folder",
  regex: /^.*$/,
  next: () => [filesOrFolders],
};

const lsOptions: Group = {
  suggestions: [
    {
      name: "-a",
      regex: /^-a$/,
    },
    {
      name: "-l",
      regex: /^-l$/,
    },
  ],
  next: () => [lsOptions, filesOrFolders],
};

const cdFolders: DynamicGroup = {
  suggestions: [
    {
      name: "folder",
      regex: /^.*$/,
      next: () => [cdFolders],
    },
  ],
  script: "ls -d */",
  postProcessor: (line) => {
    return {
      name: line,
      regex: new RegExp("^" + line + "$"),
      next: () => [cdFolders],
    };
  },
};

const cdOptions: Group = {
  suggestions: [
    {
      name: "-a",
      regex: /^-a$/,
    },
    {
      name: "-l",
      regex: /^-l$/,
    },
  ],
  next: () => [cdOptions, cdFolders],
};

const branchNames: DynamicGroup = {
  suggestions: [
    {
      name: "branch",
      regex: /^.*$/,
    },
  ],
  script: "git for-each-ref --format='%(refname:short)' refs/heads/",
  postProcessor: (line) => {
    return {
      name: line,
      regex: new RegExp("^" + line + "$"),
    };
  },
};

const gitSubCommands: Group = {
  suggestions: [
    {
      name: "checkout",
      regex: /^checkout$/,
      next: () => [branchNames],
    },
  ],
};

const clis: Group = {
  suggestions: [
    {
      name: "ls",
      regex: /^ls$/,
      next: () => [lsOptions, filesOrFolders],
    },
    {
      name: "cd",
      regex: /^cd$/,
      next: () => [cdOptions, cdFolders],
    },
    {
      name: "git",
      regex: /^git$/,
      next: () => [gitSubCommands],
    },
  ],
};

export default clis;
