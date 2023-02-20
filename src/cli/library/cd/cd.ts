import type { DynamicGroup, Group } from "../contract";

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

export const cd = {
  names: "cd",
  regex: /^cd$/,
  next: () => [cdOptions, cdFolders],
};
