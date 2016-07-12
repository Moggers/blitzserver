<script>
	$(document).on('ready', function(e) {
		var maps = [];
		<?php foreach( $maps as $id=>$map ) { ?>
		maps[<?=$map->id . '] = "' . trim($map->name) . '"'?>;
		<?php } ?>
		$('.mapgrid .pane').tooltip({container: 'body'});
		$('.mapgrid input[type=radio]').on('change', function(e) {
			$('#mapgrid-select').val( e.currentTarget.value );
			$('#mapgrid-button').html( maps[e.currentTarget.value] );
			var inst = $('[data-remodal-id="mapgrid-modal"]').remodal();
			inst.close();
		});
	});
</script>
<?=$this->Html->css('mapgrid.css');?>
<div class="remodal" data-remodal-id="mapgrid-modal">
	<div class="mapgrid col-md-12 col-sm-12">
		<h3>Select Map</h3>
		<div class="btn-group" data-toggle="buttons">
			<?php foreach( $maps as $id=>$map) { ?>
			<label class="btn btn-default pane" data-html="true" data-trigger="hover" ata-toggle="tooltip" data-placement="top" title="Name: <?=$map->name?><br />Provinces: <?=$map->prov?>">
				<input type="radio" name="memes" value="<?=$map->id?>">
					<span class="helper"></span>
					<img src="/img/maps/<?=$map->id?>/thumb256.jpeg"></img>
				</input>
			</label>
			<?php } ?>
		</div>
	</div>
</div>
