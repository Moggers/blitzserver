<script>
	$(document).on( 'ready', function() {

		// The renderer will create a canvas element for you that you can then insert into the DOM.
		//document.body.appendChild(renderer.view); // You need to create a root container that will hold the scene you want to draw.
		var stage = new PIXI.Graphics();
		var topstage = new PIXI.Graphics();

		// Provinces
		var provinces = [];

		// load the texture we need
		var renderer = {};
		var dom4;
		PIXI.loader.add('dom4', '/img/maps/'+<?=$match->map->id?>+'/map.png').load(function (loader, resources) {
			// Initialize canvas
			renderer = new PIXI.CanvasRenderer(resources.dom4.texture.width, resources.dom4.texture.height );
			$('#mapview').append( renderer.view );

			// This creates a texture from a 'dom4.png' image.
			dom4 = new PIXI.Sprite(resources.dom4.texture);

			// Position
			dom4.position.x = 0;
			dom4.position.y = 0;
			// Scale
			dom4.scale.x = 1;
			dom4.scale.y = 1;
		
			// Add the dom4 to the scene we are building.
			stage.addChild(dom4);

			renderer.render(stage);

			// kick off the animation loop (defined below)
			requestAnimationFrame(initialize);
		});

		//////
		// Province structure
		// 0 X
		// 1 Y
		// 2 Owner
		// 3 Lower gradient
		// 4 Upper gradient
		//////

		function initialize() {
			// Grab canvas and its context
			var canvas = $('#mapview canvas');
			var ctx = canvas[0].getContext('2d');
			// Color array for nation
			var cols = [];

			// Find provinces
			var dat = ctx.getImageData( 0, 0, renderer.width, renderer.height );
			for( var y = canvas[0].height; y > 0; --y ) {
				for( var x = 0; x < canvas[0].width; ++x ) {
					var id = (x + y*renderer.width)*4;
					if( (dat.data[id+0] & dat.data[id+1] & dat.data[id+2] & 255) == 255 ) {
						provinces.push( {'x':x, 'y':y, 'o':1} );
						var text = new PIXI.Text(""+provinces.length);
						text.position.x = x;
						text.position.y = y;
						text.scale.x = 0.4;
						text.scale.y = 0.4;
						stage.addChild(text);
					}
				}
			}

			// Read json and retrieve province owners
			for( var tt = 0; tt <= <?=$turnid?>; tt++ ) {
				$.ajax({
					url: "/json/"+<?=$match->id?>+"/"+tt+".json",
					dataType:'json',
					async: false,
					success: function( data ) {
						for( var ii = 0; ii < provinces.length; ii++ ) {
							if( data.provinces[ii+1] ) {
								provinces[ii].o = data.provinces[ii+1];
								if( cols[provinces[ii].o] == null ) { 
									cols[provinces[ii].o] = Math.random() * 16777215;
								}
							}
						}
					}
				});
			}
			cols[1] = 0xffffff;
			var voronoi = new Voronoi();
			var diagram = voronoi.compute( provinces, {xl:0,xr:renderer.width,yt:0,yb:renderer.height});
			console.log( diagram );
			for( var ii = 0; ii < diagram.cells.length; ii++ ) {
				var ccell = diagram.cells[ii];
				topstage.beginFill( cols[ccell.site.o], 0.3 );
				topstage.lineStyle( 1, 0xff0000, 0 );
				topstage.moveTo( ccell.halfedges[0].getStartpoint());
				topstage.lineTo( ccell.halfedges[0].getEndpoint());
				for( var kk = 0; kk < ccell.halfedges.length; kk++ ) {
					var end = ccell.halfedges[kk].getEndpoint();
					topstage.lineTo( end.x, end.y );
				}
				topstage.lineTo( ccell.halfedges[0].getStartpoint());
				topstage.endFill();
			}
			stage.addChild(topstage);
			//requestAnimationFrame(animate);
			renderer.render(stage);
		}

		function animate() {
			// start the timer for the next animation loop
			requestAnimationFrame(animate);
			
			// Draw circles around provinces
			topstage.clear();
			for( var ii = 0; ii < provinces.length; ii++ ) {
				var pos = provinces[ii];
				topstage.lineStyle( 1, 0xffffff, 255 );
				if( pos[2] != 1 ) {
					topstage.lineStyle( 1, 0xff0000, 255 );
				}
				topstage.drawCircle( pos[0], pos[1], 10 );
			}

			// this is the main render call that makes pixi draw your container and its children.
			renderer.render(stage);
			
		}
	});
</script>
	<div id="mapview" class="large-12 columns content">
</div>

