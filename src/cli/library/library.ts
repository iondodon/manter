import { cd } from "./cd/cd";
import type { Group } from "./contract";
import { git } from "./git/git";
import { mvn } from "./mvn/mvn";

const clis: Group = {
  suggestions: [cd, git, mvn],
};

export default clis;
