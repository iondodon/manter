import type { Group, Suggestion } from "../contract";

const profiles: Group = {
  suggestions: [
    {
      names: ["<profile>"],
      regex: /^.*$/,
    },
  ],
};

const mvnDOptions: Group = {
  suggestions: [
    {
      names: "maven.test.skip=",
      regex: /^maven\.test\.skip=(true|false)$/,
    },
    {
      names: "skipTests=",
      regex: /^skipTests=(true|false)$/,
    },
    {
      names: "maven.compiler.source=",
      regex: /^maven\.compiler\.source=\d+(\.\d+)?$/,
    },
    {
      names: "maven.compiler.target=",
      regex: /^maven\.compiler\.target=\d+(\.\d+)?$/,
    },
    {
      names: "maven.build.timestamp.format=",
      regex: /^maven\.build\.timestamp\.format=.*$/,
    },
  ],
  next: () => [mvnDOptions, goals, phases],
};

const phases: Group = {
  suggestions: [
    {
      names: "validate",
      regex: /^validate$/,
    },
    {
      names: "initialize",
      regex: /^initialize$/,
    },
    {
      names: "generate-sources",
      regex: /^generate-sources$/,
    },
    {
      names: "process-sources",
      regex: /^process-sources$/,
    },
    {
      names: "generate-resources",
      regex: /^generate-resources$/,
    },
    {
      names: "process-resources",
      regex: /^process-resources$/,
    },
    {
      names: "compile",
      regex: /^compile$/,
    },
    {
      names: "process-classes",
      regex: /^process-classes$/,
    },
    {
      names: "generate-test-sources",
      regex: /^generate-test-sources$/,
    },
    {
      names: "process-test-sources",
      regex: /^process-test-sources$/,
    },
    {
      names: "generate-test-resources",
      regex: /^generate-test-resources$/,
    },
    {
      names: "process-test-resources",
      regex: /^process-test-resources$/,
    },
    {
      names: "test-compile",
      regex: /^test-compile$/,
    },
    {
      names: "process-test-classes",
      regex: /^process-test-classes$/,
    },
    {
      names: "test",
      regex: /^test$/,
    },
    {
      names: "prepare-package",
      regex: /^prepare-package$/,
    },
    {
      names: "package",
      regex: /^package$/,
    },
    {
      names: "pre-integration-test",
      regex: /^pre-integration-test$/,
    },
    {
      names: "integration-test",
      regex: /^integration-test$/,
    },
    {
      names: "post-integration-test",
      regex: /^post-integration-test$/,
    },
    {
      names: "verify",
      regex: /^verify$/,
    },
    {
      names: "install",
      regex: /^install$/,
    },
    {
      names: "deploy",
      regex: /^deploy$/,
    },
  ],
  next: () => [phases],
};

const goals: Group = {
  suggestions: [
    {
      names: "clean",
      regex: /^clean$/,
    },
    {
      names: "compile",
      regex: /^compile$/,
    },
    {
      names: "test",
      regex: /^test$/,
    },
    {
      names: "package",
      regex: /^package$/,
    },
    {
      names: "install",
      regex: /^install$/,
    },
    {
      names: "deploy",
      regex: /^deploy$/,
    },
    {
      names: "site",
      regex: /^site$/,
    },
    {
      names: "generate-sources",
      regex: /^generate-sources$/,
    },
    {
      names: "generate-resources",
      regex: /^generate-resources$/,
    },
    {
      names: "test-compile",
      regex: /^test-compile$/,
    },
    {
      names: "process-test-resources",
      regex: /^process-test-resources$/,
    },
    {
      names: "surefire:test",
      regex: /^surefire:test$/,
    },
    {
      names: "failsafe:integration-test",
      regex: /^failsafe:integration-test$/,
    },
    {
      names: "dependency:tree",
      regex: /^dependency:tree$/,
    },
  ],
  next: () => [goals, phases],
};

const options: Group = {
  suggestions: [
    {
      names: ["-am", "--also-make"],
      regex: /^(-am|--also-make)$/,
    },
    {
      names: ["-amd", "--also-make-dependents"],
      regex: /^(-amd|--also-make-dependents)$/,
    },
    {
      names: ["-B", "--batch-mode"],
      regex: /^(-B|--batch-mode)$/,
    },
    {
      names: ["-b", "--builder"],
      regex: [/^-b$/, /^--builder$/],
      next: () => [{ names: ["<id>"], regex: /^.*$/ }],
    },
    {
      names: ["-C", "--strict-checksums"],
      regex: /^(-C|--strict-checksums)$/,
    },
    {
      names: ["-c", "--lax-checksums"],
      regex: /^(-c|--lax-checksums)$/,
    },
    {
      names: ["-cpu", "--check-plugin-updates"],
      regex: /^(-cpu|--check-plugin-updates)$/,
    },
    {
      names: ["-D", "--define"],
      regex: /^(-D|--define)$/,
      next: () => [{ names: ["<property>"], regex: /^.*$/ }],
    },
    {
      names: ["-e", "--errors"],
      regex: /^(-e|--errors)$/,
    },
    {
      names: ["-emp", "--encrypt-master-password"],
      regex: /^(-emp|--encrypt-master-password)$/,
      next: () => [{ names: ["<arg>"], regex: /^.*$/ }],
    },
    {
      names: ["-ep", "--encrypt-password"],
      regex: /^(-ep|--encrypt-password)$/,
      next: () => [{ names: ["<arg>"], regex: /^.*$/ }],
    },
    {
      names: ["-f", "--file"],
      regex: /^(-f|--file)$/,
      next: () => [{ names: ["<file>"], regex: /^.*$/ }],
    },
    {
      names: ["-fae", "--fail-at-end"],
      regex: /^(-fae|--fail-at-end)$/,
    },
    {
      names: ["-ff", "--fail-fast"],
      regex: /^(-ff|--fail-fast)$/,
    },
    {
      names: ["-fn", "--fail-never"],
      regex: /^(-fn|--fail-never)$/,
    },
    {
      names: ["-gs", "--global-settings"],
      regex: /^(-gs|--global-settings)$/,
      next: () => [{ names: ["<file>"], regex: /^.*$/ }],
    },
    {
      names: ["-gt", "--global-toolchains"],
      regex: /^(-gt|--global-toolchains)$/,
      next: () => [{ names: ["<file>"], regex: /^.*$/ }],
    },
    {
      names: ["-h", "--help"],
      regex: /^(-h|--help)$/,
    },
    {
      names: ["-llr", "--legacy-local-repository"],
      regex: /^(-llr|--legacy-local-repository)$/,
    },
    {
      names: ["-l", "--log-file"],
      regex: /^(-l|--log-file)$/,
      next: () => [{ names: ["<file>"], regex: /^.*$/ }],
    },
    {
      names: ["-N", "--non-recursive"],
      regex: /^(-N|--non-recursive)$/,
    },
    {
      names: ["-npr", "--no-plugin-registry"],
      regex: /^(-npr|--no-plugin-registry)$/,
    },
    {
      names: ["-npu", "--no-plugin-updates"],
      regex: /^(-npu|--no-plugin-updates)$/,
    },
    {
      names: ["-nsu", "--no-snapshot-updates"],
      regex: /^(-nsu|--no-snapshot-updates)$/,
    },
    {
      names: ["-ntp", "--no-transfer-progress"],
      regex: /^(-ntp|--no-transfer-progress)$/,
    },
    {
      names: ["-o", "--offline"],
      regex: /^(-o|--offline)$/,
    },
    {
      names: ["-P", "--activate-profiles"],
      regex: /^(-P|--activate-profiles)$/,
      next: () => [profiles],
    },
    {
      names: ["-pl", "--projects"],
      regex: /^(-pl|--projects)$/,
      next: () => [{ names: ["<arg>"], regex: /^.*$/ }],
    },
    {
      names: ["-q", "--quiet"],
      regex: /^(-q|--quiet)$/,
    },
    {
      names: ["-rf", "--resume-from"],
      regex: /^(-rf|--resume-from)$/,
      next: () => [{ names: ["<arg>"], regex: /^.*$/ }],
    },
    {
      names: ["-s", "--settings"],
      regex: /^(-s|--settings)$/,
      next: () => [{ names: ["<file>"], regex: /^.*$/ }],
    },
    {
      names: ["-T", "--threads"],
      regex: /^(-T|--threads)$/,
      next: () => [{ names: ["<arg>"], regex: /^.*$/ }],
    },
    {
      names: ["-t", "--toolchains"],
      regex: /^(-t|--toolchains)$/,
      next: () => [{ names: ["<file>"], regex: /^.*$/ }],
    },
    {
      names: ["-U", "--update-snapshots"],
      regex: /^(-U|--update-snapshots)$/,
    },
    {
      names: ["-up", "--update-plugins"],
      regex: /^(-up|--update-plugins)$/,
    },
    {
      names: ["-v", "--version"],
      regex: /^(-v|--version)$/,
    },
    {
      names: ["-V", "--show-version"],
      regex: /^(-V|--show-version)$/,
    },
    {
      names: ["-X", "--debug"],
      regex: /^(-X|--debug)$/,
    },
  ],
};

export const mvn: Suggestion = {
  names: "mvn",
  regex: /^mvn$/,
  next: () => [options, mvnDOptions, goals, phases],
};
