import clis from "../cli/library/library";
import { getByScript } from "../suggestions/GetByScript";

export const resolveDynamicGroups = async (sessionContext) => {
  for (const suggestion of sessionContext["suggestions"]) {
    if (suggestion.script) {
      try {
        suggestion["suggestions"] = await getByScript(
          suggestion,
          sessionContext
        );
      } catch (e) {
        console.log(e);
        suggestion["suggestions"] = [];
      }
    }
  }
};

export const getSuggestions = (sessionContext) => {
  let next: any = [clis];
  let words = sessionContext["lineText"]
    .trim()
    .split(" ")
    .filter((word) => word.length > 0);

  for (const word of words) {
    let found = false;
    for (const item of next) {
      if (item.suggestions) {
        for (const suggestion of item.suggestions) {
          if (suggestion.regex.test(word)) {
            if (suggestion.next) next = suggestion.next();
            else if (item.next) next = item.next();
            else next = [];
            found = true;
            break;
          }
        }
      } else if (item.regex) {
        // single suggestion
        const suggestion = item;
        if (suggestion.regex.test(word)) {
          if (suggestion.next) next = suggestion.next();
          else next = [];
          found = true;
          break;
        }
      } else {
        console.log("yoklmn");
      }

      if (found) {
        break;
      }
    }
  }

  return next;
};
