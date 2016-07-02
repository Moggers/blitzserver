<script>
	$(document).on( 'ready', function(e) {
		$(document).on('confirmation', '#swm', function(e) {
			$('#hiddenday').val( $('#modalday').val() );
			$('#hiddenhour').val( $('#modalhour').val() );
			$('#weekform').submit();
		});
		$(document).on('confirmation', '#sim', function(e) {
			$('#hiddeninthour').val($('#modalinthour').val() );
			$('#hiddenintminute').val($('#modalintminute').val() );
			$('#intervalform').submit();
		});
		$('#intervalform').ajaxForm( function(res) {
			var data = JSON.parse( res );
			if( data.status == 1 ) {
				window.location = '/matches/view/'+data.id;
			} else {
				window.location.hash = "modal";
			}
		});
		$('#weekform').ajaxForm( function(res) {
			var data = JSON.parse( res );
			if( data.status == 1 ) {
				window.location = '/matches/view/'+data.id;
			} else {
				window.location.hash = "modal";
			}
		});
		$('#emailform').ajaxForm( function(res) {
			console.log( "wew" );
			var data = JSON.parse(res);
			if( data.status == 0 ) {
				alert( "Roger!");
			} else {
				alert( "OH NOOO");
			}
		});
		if( $('#localtime').length ) {
			var time = moment.tz($('#localtime')[0].innerHTML, 'YYYY-MM-DD HH:nm:ss', 'Europe/Dublin').tz( moment.tz.guess())
			$('#localtime')[0].innerHTML = moment.preciseDiff( moment(), time);
		}
		$('#aination').on('change', function(e)
		{
			$.post('../markcomputer', {'id': <?=$match->id?>, 'nation_id': $('#aination').val()}, function(res) {
				var data = JSON.parse(res);
				if( data.status == 0 ){
					alert('Done');
				} if( data.status == 1 ) {
					window.location.hash = "modal";
				} if( data.status == 2 ){
					alert('Unknown error');
				}
			});
		});
		$('#turndelay').on('change', function(e)
		{
			$.post('../turndelay', {'id': <?=$match->id?>, 'turndelay': $('#turndelay').val()}, function(res) {
				var data = JSON.parse(res);
				if( data.status == 0 ){
					alert('Done');
				} if( data.status == 1 ) {
					window.location.hash = "modal";
				} if( data.status == 2 ){
					alert('Unknown error');
				}
			});
		});

	});
</script>

<div class="matches view large-12 columns content">
    <h3><?= h($match->name) ?></h3>
	<div class="wrapper">
		<div class="maptable">
			<div class="large-6 medium-12 small-12 columns content">
				<table class="vertical-table" style="background:#fafafa" >
					<tr>
						<th><?= __('Thrones(Points To Win)') ?></th>
						<td><?= $match->thrones ?>
					<tr>
						<th><?= __('Age') ?></th>
						<td><?= $match::ages( $match->age ) ?></td>
					</tr>
					<tr>
						<th><?= __('Magic Sites') ?></th>
						<td><?= $match->siterarity ?></td>
					</tr>

					<tr>
						<th><?= __('Address') ?></th>
						<td><?= $match->address ?></td>
					</tr>
						<th><?= __('Status') ?> </th>
						<td><?= $match->status_string ?></td>
					<tr>
						<?php if( $match->status == 3 ) { ?>
							<?php if( $match->next_turn !== false ) { ?>
								<th><?= __('Next Turn') ?> </th>
								<td id='localtime'><?= $match->next_turn->i18nFormat() ?></th>
							<?php } ?>
						<?php } ?>
					</tr>
				</table>
				<table class="vertical-table" style="background:#fafafa">
				<th>Players</th>
				<?php if( $match->status !== 3 ) {
					echo "<th>Actions</th>";
				} else {
					echo "<th>Turn Submisssions</th>";
				} ?>
				<?php foreach ($match->nations as $nation): ?>
					<tr>
						<td>
						<?=$nation['_joinData']['name']?> <br /><span style="font-size:11px;color:grey"><?= $nation['name']. ', ' .$nation['subtitle']?></style><?php if( $nation->_joinData->computer == 1 ) { echo ' <span style="color:blue">AI</span>';}?>
						</td>
						<?php if( $match->status !== 3 ) { ?>
								<td>
							<?php if( $nation->_joinData->markdelete == 0 ) { ?>
								<?= $this->Html->link(__('Remove'), ['controller' => 'Matches', 'action' => 'removePlayer', $nation->_joinData->id ]) ?>
							<?php } else { ?>
								<?= "<b>(Removing..)</b>" ?>
							<?php } ?>
							</td>
						<?php } else {
							echo "<td style='overflow:hidden'>";
							echo "<div style='letter-spacing: -1px; float:right'>";
							foreach( $match->turns as $turn ):
								$found = 0;
								foreach( $turn->matchnationturns as $mnt ):
									if( $mnt->matchnation_id == $nation->_joinData->id ) {
										$found = 1;
									} else if( $nation->_joinData->markcomputer == 1 ) {
										$found = 2;
									}
								endforeach;
								if( $found == 0 ) {
									echo '<span style="color:red">|</span>';
								} else if( $found == 1 ){
									echo '<span style="color:green">|</span>';
								} else if( $found == 2 ){
									echo '<span style="color:blue">|</span>';
								}
							endforeach;
							echo "</div>";
							echo "</td>";
						}?>
					</tr>
				<?php endforeach; ?>
				</table>
				<?= $this->Form->create( $match, ['id' => 'emailform', 'url' => ['action' => 'requestnotify']] ); ?>
					<h5>New Turn Notification</h5>
					<i>Only enter the address if you want to be told of new turns, enter your nation and the time before rollover, if you want to warned about staling</i>
					<div class='large-12 medium-12 small-12 columns content nopad'>
						<?= $this->Form->input('email', ['label' => false, 'placeholder' => 'Email Address'] ); ?>
					</div>
					<div class='large-6 medium-6 small-12 columns content nopad'>
						<?= $this->Form->input('hours', ['label' => false, 'placeholder' => 'Hours until host'] ); ?>
					</div>
					<div class='large-6 medium-6 small-12 columns content nopad'>
						<?= $this->Form->select('matchnation_id', $nations, ['label' => false, 'empty' => 'Nation' ] ); ?>
					</div>
					<?= $this->Form->button(__('Submit')); ?>
				<?= $this->Form->end(); ?>
			</div>
			<div class="large-6 medium-12 small-12 columns content">
				<div class="large-6 medium-6 small-6 columns content">
					<?= $this->Html->link( 'Schedule by Time of Week', '#scheduleweek', ['class' => 'button']); ?>
					<?= $this->Form->create( $match, ['id' => 'weekform', 'url' => ['action' => 'weekschedule']] ); ?>
						<?= $this->Form->hidden('day', ['id' => 'hiddenday']); ?>
						<?= $this->Form->hidden('hour', ['id' => 'hiddenhour']); ?>
					<?= $this->Form->end(); ?>
				</div>
				<div class="large-6 medium-6 small-6 columns content">
					<?= $this->Html->link( 'Schedule by Host Interval', '#scheduleinterval',['class' => 'button']); ?>
					<?= $this->Form->create($match, ['id' => 'intervalform', 'url' => ['action' => 'hostinterval']]); ?>
						<?= $this->Form->hidden('hour', ['id' => 'hiddeninthour']); ?>
						<?= $this->Form->hidden('minute', ['id' => 'hiddenintminute']); ?>
					<?= $this->Form->end(); ?>
				</div>
				<?= $this->Form->create($match, ['id' => 'settingschangeform', 'url' => ['action' => 'edit']]) ?>
				<div class="large-12 medium-12 small-12 columns content">
					<div class="large-4 medium-4 small-12 columns content">
						<?php if( $match->status != 3 ) {
							echo $this->Form->input('tone',
								['label' => 'T1 Thrones', 'default' => $match->tone ]);
							echo $this->Form->input('ttwo',
								array('label' => 'T2 Thrones', 'default' => $match->ttwo ));
							echo $this->Form->input('tthree',
								array('label' => 'T3 Thrones', 'default' => $match->tthree ));
							echo $this->Form->input('points',
								array('label' => 'Points To Win', 'default' => $match->points ));
						} else {
							echo $this->Form->input( 'maxholdups' );
						} ?>
					</div>
					<div class="large-8 medium-8 small-12 columns content">
						<?php if( $match->status != 3 ) {
							echo $this->Form->input('map_id', [ 'default' => $match->map_id, 'options' => $maps->where(['hide' => 0])]);
							echo $this->Form->input( 'research_diff', [
								'options' => [-1 => "Very Easy", 0 => "Easy", 1 => "Normal", 2 => "Hard", 3 => "Very Hard"],
								'value' => $match->research_diff,
								'label' => 'Research Difficulty']);
							echo $this->Form->input( 'renaming', array(
								'label' => 'Commander Renaming',
								'type' => 'checkbox'  ));
							echo $this->Form->input( 'clientstart', array(
								'label' => 'Clients Can Start Game',
								'type' => 'checkbox'  ));
							} else{
						} ?>
						<?= $this->Form->button(__('Submit')) ?>
					</div>
				</div>
				<?= $this->Form->end() ?>
				<?= $this->Form->select('ainations', $allnations, ['id'=>'aination','style'=>'position:relative', 'empty' => 'Set Nation AI']); ?>
			</div>
			<?= $this->element( 'modtable', array( 'mods' => $match->mods )); ?>
		</div>
		<div id="mapview" class="mappreview">
			<?= $this->element('mapvoronoi', array('match' => $match )); ?>
			<?= $this->Form->input('turndelay', ['default' => $match->turndelay, 'label' => 'Turn Delay', 'id' => 'turndelay']); ?>
			<?= $this->element('postview', [ 'posts' => $match->posts, 'newpost' => $newpost] ); ?>
		</div>
	</div>
	<div style="width:300px" class="remodal" id="swm" data-remodal-id="scheduleweek">
		<button data-remodal-action="close" class="remodal-close"></button>
				<div class="large-8 medium-8 small-8 columns content nopad">
					<?= $this->Form->input( 'day', [ 'id' => 'modalday', 'empty' => 'day', 'label' => false, 'options' => [
						0 => 'Sunday', 1 => 'Monday', 2 => 'Tuesday', 3 => 'Wednesday',
						4 => 'Thursday', 5 => 'Friday', 6 => 'Saturday']] ); ?>
				</div>
				<div class="large-4 medium-4 small-4 columns content nopad">
					<?= $this->Form->input( 'hour', [ 'id' => 'modalhour', 'empty' => 'hour', 'label' => false, 'options' => [ 
						0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 
						13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23 ]] ); ?>
				</div>
		<button data-remodal-action="cancel" class="remodal-cancel">Cancel</button>
		<button data-remodal-action="confirm" class="remodal-confirm">Apply</button>
	</div>
	<div style="width:200px" class="remodal" id="sim" data-remodal-id="scheduleinterval">
		<button data-remodal-action="close" class="remodal-close"></button>
				<div class="large-6 medium-6 small-6 columns content nopad">
					<?= $this->Form->input( 'hour', ['id' => 'modalinthour'] ); ?>
				</div>
				<div class="large-6 medium-6 small-6 columns content nopad">
					<?= $this->Form->input( 'minute', ['id' => 'modalintminute'] ); ?>
				</div>
		<button data-remodal-action="cancel" class="remodal-cancel">Cancel</button>
		<button data-remodal-action="confirm" class="remodal-confirm">Apply</button>
	</div>
</div>

