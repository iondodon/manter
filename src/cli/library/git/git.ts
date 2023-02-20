import type { DynamicGroup, Group } from "../contract";

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

export const git = {
  names: "git",
  regex: /^git$/,
  next: () => [gitSubCommands],
};
