Feature('maps');

Scenario('upload map', async ({I}) => {
	I.amOnPage("/maps");
	I.see("Upload Map");
	I.attachFile("map", "./test_data/Isle_of_Avalon.map");
	I.attachFile("tga", "./test_data/Isle of Avalon.tga");
	I.click("Upload");
	I.see("Isle of Avalon");
});
