Feature('games');
const faker = require('faker');

Scenario('upload new map and create game', async ({I}) => {
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
	let address = await I.grabTextFrom(".pane.status tr:first-child td:nth-child(2)");
	I.connectToServer(address.split(":")[0], address.split(":")[1], game_name);
});
