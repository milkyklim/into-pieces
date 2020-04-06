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

const PASTE_PARAMS = {
  title: "First paste of all",
  text: "Have you ever tried into pieces?",
  language: "None",
  timestamp: 1580137056,
  expiration: 1580167056,
};

orchestrator.registerScenario(
  "alice can add, retrieve and remove her paste",
  async (s, t) => {
    const { alice } = await s.players({ alice: config }, true);

    const createResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "create_paste",
      PASTE_PARAMS,
    );
    t.ok(createResult.Ok);

    const pasteAddress = createResult.Ok;

    const retrieveResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "get_paste",
      { address: pasteAddress },
    );
    const paste = JSON.parse(retrieveResult.Ok.App[1]);
    t.deepEqual(paste, {
      ...PASTE_PARAMS,
      author_id: alice.instance("into_pieces").agentAddress,
      reported: false,
    });

    const removeResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "remove_paste",
      { address: pasteAddress },
    );
    t.ok(removeResult.Ok);
  },
);

orchestrator.run();
