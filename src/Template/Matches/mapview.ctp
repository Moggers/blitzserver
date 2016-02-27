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

		function initialize() {
			// Find those pixels
			var canvas = $('#mapview canvas');
			var ctx = canvas[0].getContext('2d');
			var dat = ctx.getImageData( 0, 0, renderer.height, renderer.width );
			var borders = ctx.getImageData( 0, 0, renderer.height, renderer.width );
			var cols = [];
			Uint16Array.prototype.pih = 0;
			Uint16Array.prototype.pit = 0;
			Uint16Array.prototype.push = function(n) {
				this[this.pih] = n;
				this.pih++;
				this.pih %= this.length;
			}
			Uint16Array.prototype.shift = function() {
				if( this.pit == this.pih ) return;
				var ret = this[this.pit];
				this.pit++;
				this.pit %= this.length;
				return ret;
			}
			var pixiterator = new Uint16Array(256000000);
			for( var y = canvas[0].height; y > 0; --y ) {
				for( var x = 0; x < canvas[0].width; ++x ) {
					var id = (x + y*renderer.width)*4;
					if( (dat.data[id+0] & dat.data[id+1] & dat.data[id+2] & 255) == 255 ) {
						provinces.push( [x, y, 1] );
					}
				}
			}

			for( var tt = 0; tt <= <?=$turnid?>; tt++ ) {
				$.ajax({
					url: "/json/"+<?=$match->id?>+"/"+tt+".json",
					dataType:'json',
					async: false,
					success: function( data ) {
						for( var ii = 0; ii < provinces.length; ii++ ) {
							if( data.provinces[ii+1] ) {
								provinces[ii][2] = data.provinces[ii+1];
								if( cols[provinces[ii][2]] == null ) { 
									cols[provinces[ii][2]] = [Math.random()*255, Math.random()*255, Math.random()*255];
								}
							}
						}
					}
				});
			}
			for( var tt = 0; tt < provinces.length; tt++ ) {
				pixiterator.push( provinces[tt][0] );
				pixiterator.push( provinces[tt][1] );
				pixiterator.push( provinces[tt][2] );
			}
			renderer.render(stage);
			stage.addChild(topstage);
			requestAnimationFrame(animate);
			var tt = [];
			var ii = 0;
			while( (tt[0] = pixiterator.shift())!=null) {
				tt[1] = pixiterator.shift();
				tt[2] = pixiterator.shift();
				var ix = (tt[0]+tt[1]*borders.width)*4;
				if( borders.data[ix+3] == 255 ) {
					if( tt[2] == 1 ) {
						borders.data[ix+3] = 0;
					} else {
						borders.data[ix] = cols[tt[2]][0];
						borders.data[ix+1] = cols[tt[2]][1];
						borders.data[ix+2] = cols[tt[2]][2];
						borders.data[ix+3] = 100;
					}
					pixiterator.push( tt[0] );
					pixiterator.push( tt[1]-1 );
					pixiterator.push( tt[2] );
					pixiterator.push( tt[0] );
					pixiterator.push( tt[1]+1 );
					pixiterator.push( tt[2] );
					pixiterator.push( tt[0]-1 );
					pixiterator.push( tt[1] );
					pixiterator.push( tt[2] );
					pixiterator.push( tt[0]+1 );
					pixiterator.push( tt[1] );
					pixiterator.push( tt[2] );
				}
			}
			console.log( 'done' );
			var tempcanvas = document.createElement('canvas');
			tempcanvas.height = renderer.height;
			tempcanvas.width = renderer.width;
			tempcanvas.getContext('2d').putImageData( borders, 0, 0 );
			var canvastext = PIXI.Texture.fromCanvas( tempcanvas );
			var spr = new PIXI.Sprite( canvastext );
			stage.addChild( spr );
		}

		function animate() {
			// start the timer for the next animation loop
			requestAnimationFrame(animate);
			
			// Draw circles around provinces
			topstage.clear();
			for( var ii = 0; ii < provinces.length; ii++ ) {
				var pos = provinces[ii];
				topstage.lineStyle( 1, 0xffffff, 255 );
				if( pos.owner ) {
					topstage.lineStyle( 1, 0xff0000, 255 );
				}
				topstage.drawCircle( pos.x, pos.y, 10 );
			}

			// this is the main render call that makes pixi draw your container and its children.
			renderer.render(stage);
			
		}
	});
</script>
	<div id="mapview" class="large-12 columns content">
</div>

