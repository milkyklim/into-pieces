/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor,
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/into-pieces.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,
  ),
});

const dna = Config.dna(dnaPath, "into_pieces");
const config = Config.gen(
  {
    into_pieces: dna,
  },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000",
    },
    logger: Config.logger({ type: "error" }),
  },
);

orchestrator.registerScenario("Test hello holo", async (s, t) => {
  const { alice, bob } = await s.players({ alice: config, bob: config }, true);

  const message = "Some text";
  const timestamp = 1580137056;

  const result = await alice.call(
    "into_pieces",
    "into_pieces",
    "hello_holo",
    {},
  );

  t.ok(result.Ok);
  t.deepEqual(result, { Ok: "Hello Holo" });

  const create_result = await alice.call(
    "into_pieces",
    "into_pieces",
    "create_post",
    {
      message: message,
      timestamp: timestamp,
    },
  );
  t.ok(create_result.Ok);
  const alice_person_address = alice.instance("into_pieces").agentAddress;

  await s.consistency();

  const retrieve_result = await bob.call(
    "into_pieces",
    "into_pieces",
    "retrieve_posts",
    { agent_address: alice_person_address },
  );

  t.ok(retrieve_result.Ok);
  t.deepEqual(retrieve_result.Ok[0], {
    message: message,
    timestamp: timestamp,
    author_id: alice_person_address,
  });
});

orchestrator.run();
