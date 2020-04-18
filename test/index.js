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

process.on("unhandledRejection", (error) => {
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

orchestrator.registerScenario("paste zome says hello", async (s, t) => {
  const { alice } = await s.players({ alice: config }, true);
  const helloResult = await alice.call(
    "into_pieces",
    "into_pieces",
    "hello_holo",
    {},
  );

  t.deepEqual(helloResult.Ok, "Hello Holo");
});

orchestrator.registerScenario(
  "alice can create, retrieve and remove her paste",
  async (s, t) => {
    const { alice } = await s.players({ alice: config }, true);
    const params = { ...PASTE_PARAMS };

    const createResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "create_paste",
      params,
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
      ...params,
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

orchestrator.registerScenario("alice can update her paste", async (s, t) => {
  const { alice } = await s.players({ alice: config }, true);
  const params = { ...PASTE_PARAMS };
  const createResult = await alice.call(
    "into_pieces",
    "into_pieces",
    "create_paste",
    params,
  );

  let pasteAddress = createResult.Ok;
  const newPasteParams = { ...PASTE_PARAMS };
  newPasteParams.title = "That's the second paste!";
  newPasteParams.language = "Plain";

  const updateResult = await alice.call(
    "into_pieces",
    "into_pieces",
    "update_paste",
    {
      address: pasteAddress,
      ...newPasteParams,
    },
  );

  t.ok(updateResult.Ok);

  pasteAddress = updateResult.Ok;
  const retrieveResult = await alice.call(
    "into_pieces",
    "into_pieces",
    "get_paste",
    { address: pasteAddress },
  );
  const paste = JSON.parse(retrieveResult.Ok.App[1]);
  t.deepEqual(paste, {
    ...newPasteParams,
    reported: false,
  });
});

orchestrator.registerScenario(
  "bob can retrieve alice's paste",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: config, bob: config },
      true,
    );

    const params = { ...PASTE_PARAMS };
    const createResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "create_paste",
      params,
    );
    await s.consistency();

    const pasteAddress = createResult.Ok;
    const retrieveResult = await bob.call(
      "into_pieces",
      "into_pieces",
      "get_paste",
      { address: pasteAddress },
    );
    const paste = JSON.parse(retrieveResult.Ok.App[1]);
    t.deepEqual(paste, {
      ...params,
      reported: false,
    });
  },
);

orchestrator.registerScenario("bob can update alice's paste", async (s, t) => {
  const { alice, bob } = await s.players({ alice: config, bob: config }, true);
  const params = { ...PASTE_PARAMS };
  const createResult = await alice.call(
    "into_pieces",
    "into_pieces",
    "create_paste",
    params,
  );
  await s.consistency();

  let pasteAddress = createResult.Ok;
  const newPasteParams = { ...PASTE_PARAMS };
  newPasteParams.title = "That's the second paste!";
  newPasteParams.language = "Plain";

  const updateResult = await bob.call(
    "into_pieces",
    "into_pieces",
    "update_paste",
    {
      address: pasteAddress,
      ...newPasteParams,
    },
  );

  t.ok(updateResult.Ok);
  await s.consistency();

  pasteAddress = updateResult.Ok;
  const retrieveResult = await alice.call(
    "into_pieces",
    "into_pieces",
    "get_paste",
    { address: pasteAddress },
  );
  const paste = JSON.parse(retrieveResult.Ok.App[1]);
  t.deepEqual(paste, {
    ...newPasteParams,
    reported: false,
  });
});

orchestrator.registerScenario(
  "alice can't create post with title too long",
  async (s, t) => {
    const { alice } = await s.players({ alice: config }, true);
    const params = { ...PASTE_PARAMS };
    // title >= 50 chars
    params.title = "d".repeat(50);

    const createResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "create_paste",
      params,
    );

    const error = JSON.parse(createResult.Err.Internal);
    t.equal(error.kind.ValidationFailed, "Symbols in title above 50");
  },
);

orchestrator.registerScenario(
  "alice can't create post with text too long",
  async (s, t) => {
    const { alice } = await s.players({ alice: config }, true);
    const params = { ...PASTE_PARAMS };
    // text >= 1024 chars
    params.text = "d".repeat(1024);

    const createResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "create_paste",
      params,
    );

    const error = JSON.parse(createResult.Err.Internal);
    t.equal(error.kind.ValidationFailed, "Symbols in text above 1024");
  },
);

orchestrator.registerScenario(
  "bob can't remove alice's paste",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: config, bob: config },
      true,
    );
    const params = { ...PASTE_PARAMS };

    const createResult = await alice.call(
      "into_pieces",
      "into_pieces",
      "create_paste",
      params,
    );

    await s.consistency();

    const pasteAddress = createResult.Ok;
    const removeResult = await bob.call(
      "into_pieces",
      "into_pieces",
      "remove_paste",
      { address: pasteAddress },
    );

    const error = JSON.parse(removeResult.Err.Internal);
    t.equal(
      error.kind.ValidationFailed,
      "Agent who did not author is trying to delete",
    );
  },
);

orchestrator.run();
