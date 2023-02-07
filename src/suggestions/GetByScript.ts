import { Child, Command } from "@tauri-apps/api/shell";
import type { Suggestion } from "src/cli/library/contract";

const windows = navigator.userAgent.includes("Windows");
let cmd = windows ? "cmd" : "sh";
let args = windows ? ["/C"] : ["-c"];
let env = {
  // key: val
};
let child: Child;

function executeScript(
  dynamicGroup,
  sessionContext
): Promise<{ code: any; res: any[] }> {
  const script = dynamicGroup["script"];

  const cwd = sessionContext["prompt_command_result"]["cwd"];
  let res = [];

  child = null;
  const command = new Command(cmd, [...args, script], {
    cwd: cwd || null,
    env: env,
  });

  command.stdout.on("data", (line) => {
    res.push(dynamicGroup["postProcessor"](line));
  });

  command
    .spawn()
    .then((c) => (child = c))
    .catch((r) => console.log(r));

  command.stderr.on("data", (line) => {
    console.log("stderr " + line);
  });

  return new Promise((resolve, reject) => {
    command.on("close", (data) => {
      console.log(
        `Command finished with code ${data.code} and signal ${data.signal}`
      );
      child = null;
      if (data.code != 0) {
        reject(
          `Command finished with code ${data.code} and signal ${data.signal}`
        );
      }
      resolve({ code: data.code, res: res });
    });

    command.on("error", (error) => {
      console.log("error", error);
      reject("error " + error);
    });
  });
}

function tryExecuteScript(
  dynamicGroup,
  sessionContext
): Promise<{ code: any; res: any[] }> {
  return executeScript(dynamicGroup, sessionContext);
}

export function getByScript(
  dynamicGroup,
  sessionContext
): Promise<Suggestion[]> {
  return new Promise((resolve, reject) => {
    tryExecuteScript(dynamicGroup, sessionContext)
      .then((response) => resolve(response.res))
      .catch((err) => {
        console.log(err);
        reject(err);
      });
  });
}

function kill() {
  child
    .kill()
    .then(() => console.log("killed child process"))
    .catch((r: any) => console.log(r));
}

function writeToStdin(toWrite) {
  child.write(toWrite).catch((r: any) => console.log(r));
}
