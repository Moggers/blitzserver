<script type="text/javascript">
	$(document).ready( function() {
		$('#matchcreate').ajaxForm( function(res) {
			var data = JSON.parse( res );
			if( data.status == 0 ) {
				window.location = '/matches/view/'+data.id;
			} else if( data.status == 1 ){
				window.location.hash = "modal";
			} else {
				alert("Unknown failure! Check your options, and if you're sure everything is right, file a bug report.");
			}
		});
		$('#modtable tr').on('click', function(e) {
			if( e.delegateTarget.id.substring(0,3) == 'mod' ) {
				$('#hiddenmods > option').each( function() {
					if( this.value == e.delegateTarget.id.split('_')[1] ) {
						this.selected = !this.selected;
						if( !this.selected )
							e.delegateTarget.style.background = '';
						else 
							e.delegateTarget.style.background = '#babaff';
					}
				});
			}
		});
	});
</script>

<div class="col-md-12 col-sm-12">
    <?= $this->Form->create($match, ['id' => 'matchcreate']) ?>
    <fieldset>
		<h3>Create Match</h3>
			<div class="col-md-4 col-sm-6 fixed-input-group-thin">
				<h4>Important Stuff</h4>
				<div class="form-group">
					<div class="input-group">
						<span class="input-group-addon" id="namelabel">Name</span>
						<input aria-describedby="namelabel" type="text" class="form-control" name="name"></input>
					</div>
					<div class="input-group">
						<span class="input-group-addon" id="maplabel">Map</span>
						<a href="#mapgrid-modal" aria-describedby="maplabel" id="mapgrid-button" class="btn btn-block btn-default">Select Map</a>
					</div>
					<input type="hidden" id="mapgrid-select" name="map_id"></input>
					<div class="input-group">
						<span class="input-group-addon" id="agelabel">Age</span>
						<select class="form-control" name="age" aria-describedby="agelabel">
							<option value="1">Early</option>`
							<option value="2">Middle</option>`
							<option value="3">Late</option>`
						</select>
					</div>
					<div style="display:none">
						<?= $this->Form->input( 'mods._ids', ['type' => 'select', 'multiple' => 'true', 'options' => $mods, 'id' => 'hiddenmods' ] ); ?>
					</div>
					<a href="#modselect" class="btn btn-default btn-block">Select Mods</a>
					<div class="remodal" data-remodal-id="modselect">
						<?= $this->element( 'modtable', array( 'mods' => $modsfull )); ?>
					</div>
				</div>
			</div>
			<div class="col-md-4 col-sm-6 fixed-input-group-fat">
				<h4>Victory Conditions</h4>
				<div class="form-group throne-inputs">
					<div class="input-group">
						<span class="input-group-addon" id="tonelabel">Level One Thrones</span>
						<input type="text" class="form-control" aria-describedby="tonelabel" name="tone" value="5"/>
					</div>
					<div class="input-group">
						<span class="input-group-addon" id="ttwolabel">Level Two Thrones</span>
						<input type="text" class="form-control" aria-describedby="ttwolabel" name="ttwo" value="0"/>
					</div>
					<div class="input-group">
						<span class="input-group-addon" id="tthreelabel">Level Three Thrones</span>
						<input type="text" class="form-control" aria-describedby="tthreelabel" name="tthree" value="0"/>
					</div>
					<div class="input-group">
						<span class="input-group-addon" id="pointslabel">Points To Win</span>
						<input type="text" class="form-control" aria-describedby="pointslabel" name="points" value="5"/>
					</div>
				</div>
			</div>
			<div class="fixed-input-group-mid col-md-4 col-sm-12">
				<h4>Miscellaneous</h4>
				<div class="input-group">
					<span class="input-group-addon" id="researchlabel">Research Diff</span>
					<select name="research_diff" class="form-control" aria-dscribedby="researchlabel">
						<option value="-1">Very Easy</option>
						<option value="0">Easy</option>
						<option selected="selected" value="1">Normal</option>
						<option value="2">Hard</option>
						<option value="3">Very Hard</option>
					</select>
				</div>
				<div class="btn-group-vertical btn-block" data-toggle="buttons">
					<label class="btn btn-default">
						<input type="checkbox" name="renaming" value="1">Commander Renaming
					</label>
					<label class="btn btn-default">
						<input type="checkbox" name="clientstart" value="1">Clients Can Start Game
					</label>
				</div>
				<div class="input-group">
					<span class="input-group-addon" id="sitelabel">Magic Sites</span>
					<input type="text" class="form-control" aria-describedby="sitelabel" value="40" name="siterarity"/>
				</div>
				<div class="input-group">
					<span class="input-group-addon" id="passwordlabel">Master Password</span>
					<input type="text" class="form-control" aria-describedby="passwordlabel" name="master_password"/>
				</div>
			</div>
			<div class="fixed-input-group-mid col-md-12 col-sm-12">
				<button type="submit" class="btn btn-default btn-block">Submit</button>
			</div>
    </fieldset>
    <?= $this->Form->end() ?>
	<?= $this->element('mapgrid', ['maps' => $maps]);?>
