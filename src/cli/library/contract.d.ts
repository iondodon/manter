export type Group = {
  suggestions: Suggestion[];
  next?: () => (Group | Suggestion)[];
};

export type DynamicGroup = Group & {
  script: string;
  postProcessor: (line: string) => Suggestion;
};

export type Suggestion = {
  name: string;
  regex: RegExp;
  next?: () => (Group | Suggestion)[];
};
