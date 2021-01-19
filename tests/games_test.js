Feature("games");
const faker = require("faker");

Scenario("upload new map, new mod, and create game", async ({I}) => {
	await I.clearDatabase();
	await I.launchBlitzserver();

	// Upload Map
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/Isle_of_Avalon.map");
	I.attachFile("tga", "./test_data/Isle of Avalon.tga");
	I.click("Upload");
	I.see("Isle of Avalon");

	// Upload Mod
	I.amOnPage("/mods");
	I.see("Upload Mod");
	I.attachFile("archive", "./test_data/worthy_heroes.zip");
	I.click("Upload");
	I.see("Worthy_Heroes");

	// Create Game
	let game_name = faker.name.firstName();
	I.amOnPage("/games");
	I.see("Create New Game");
	I.fillField("name", game_name);
	I.fillField("password", "password");
	I.click("Create");
	I.seeInCurrentUrl("settings");
	I.click("Worthy_Heroes");
	I.click("Save Settings");

	// Connect to game
	I.amOnPage("/game/" + game_name + "/status");
	let address = await I.grabTextFrom(".pane.status tr:nth-child(2) td:nth-child(2)");
	I.connectToServer(address.split(":")[0], address.split(":")[1], game_name);
});

Scenario("set countdown", async ({I}) => {
	await I.clearDatabase();
	await I.launchBlitzserver();

	// Upload Map
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/Isle_of_Avalon.map");
	I.attachFile("tga", "./test_data/Isle of Avalon.tga");
	I.click("Upload");
	I.see("Isle of Avalon");

	// Create Game
	let game_name = faker.name.firstName();
	I.amOnPage("/games");
	I.see("Create New Game");
	I.fillField("name", game_name);
	I.fillField("password", "password");
	I.click("Create");
	I.seeInCurrentUrl("settings");

	// Schedule game
	I.click("schedule");
	I.fillField("countdown", 60);
	I.click("Begin Countdown");

	// Connect to game
	I.amOnPage("/game/" + game_name + "/status");
	let address = await I.grabTextFrom(".pane.status tr:nth-child(2) td:nth-child(2)");
	I.connectToServer(address.split(":")[0], address.split(":")[1], game_name);
});
Scenario("archive game", async ({I}) => {
	await I.clearDatabase();
	await I.launchBlitzserver();

	// Upload Map
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/Isle_of_Avalon.map");
	I.attachFile("tga", "./test_data/Isle of Avalon.tga");
	I.click("Upload");
	I.see("Isle of Avalon");

	// Create Game
	let game_name = faker.name.firstName();
	I.amOnPage("/games");
	I.see("Create New Game");
	I.fillField("name", game_name);
	I.fillField("password", "password");
	I.click("Create");
	I.seeInCurrentUrl("settings");

	// Connect to game
	I.amOnPage("/game/" + game_name + "/status");
	let address = await I.grabTextFrom(".pane.status tr:nth-child(2) td:nth-child(2)");
	I.connectToServer(address.split(":")[0], address.split(":")[1], game_name);

	// Archive game
	I.amOnPage("/game/" + game_name + "/schedule");
	I.click("Archive Game");
	I.cannotConnectToServer(address.split(":")[0], address.split(":")[1]);

	// Create another game and check it has the same port
	let new_game_name = faker.name.firstName();
	I.amOnPage("/games");
	I.see("Create New Game");
	I.fillField("name", new_game_name);
	I.fillField("password", "password");
	I.click("Create");
	I.seeInCurrentUrl("settings");
	I.amOnPage("/game/" + new_game_name + "/status");
	I.see(address.split(":")[1]);
});

Scenario("validate map upload", async ({I}) => {
	await I.clearDatabase();
	await I.launchBlitzserver();

	// Upload Map
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/Isle_of_Avalon.map");
	I.attachFile("tga", "./test_data/Isle of Avalon.tga");
	I.click("Upload");
	I.see("Isle of Avalon");

	// Upload Map
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/Isle_of_Avalon.map");
	I.attachFile("tga", "./test_data/islandsofgoodandevil.tga");
	I.click("Upload");
	I.see("Map #imagefile is Isle of Avalon.tga, but TGA filename is");

	// Upload Map
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/Isle_of_Avalon.map");
	I.attachFile("tga", "./test_data/Isle of Avalon.tga");
	I.attachFile("tga_winter", "./test_data/islandsofgoodandevil_winter.tga");
	I.click("Upload");
	I.see("Map does not contain a #winterimagefile, but islandsofgoodandevil_winter.tga has been uploaded as one");
	//
	// Upload Map
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/islandsofgoodandevil.map");
	I.attachFile("tga", "./test_data/islandsofgoodandevil.tga");
	I.attachFile("tga_winter", "./test_data/islandsofgoodandevil_winter.tga");
	I.click("Upload");
	I.see("Islands of Good and Evil");
});
