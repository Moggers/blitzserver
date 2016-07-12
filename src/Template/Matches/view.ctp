<script>
	var matchid = <?=$match->id?>;
	var matchstatus = <?=$match->status?>;
</script>
<?= $this->Html->script('matchview.js'); ?>

<div class="matches view col-md-12">
	<div class="match-title">
		<h3><?= h($match->name) ?>
		<span><a href="#settings-modal" class="btn btn-default"><div class="glyphicon glyphicon-pencil"></div></a></span>
		</h3>
	</div>
	<div class="wrapper">
		<div class="maptable">
			<div class="col-md-6 col-sm-12">
				<table class="table table-bordered">
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
						<i class="fa fa-desktop"></i>
						</td>
						<?php if( $match->status !== 3 && $match->status !== 70 ) { ?>
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
				<div class="col-md-12 col-sm12">
					<h5>New Turn Notification</h5>
					<i>Only enter the address if you want to be told of new turns, enter your nation and the time before rollover, if you want to warned about staling</i>
					<?= $this->Form->create( $match, ['id' => 'emailform', 'url' => ['action' => 'requestnotify']] ); ?>
						<div class="input-group">
							<input type="text" class="form-control" placeholder="Email Address" name="email"></input>
							<span class="input-group-btn"><button type="submit" class="btn btn-default">Submit</button></span>
						</div>
						<div class="form-inline">
							<input type="text" class="form-control" placeholder="Hours until host" name="hours"></input>
							<?= $this->Form->select('matchnation_id', $nations, ['class' => 'form-control', 'label' => false, 'empty' => 'Nation' ] ); ?>
						</div>
					<?= $this->Form->end(); ?>
				</div>
			</div>
			<div class="col-md-6 col-sm-12">
				<div id="mapview" class="mappreview">
					<?= $this->element('mapvoronoi', array('match' => $match )); ?>
					<?= $this->Form->input('turndelay', ['default' => $match->turndelay, 'label' => 'Turn Delay', 'id' => 'turndelay']); ?>
					<?= $this->element('postview', [ 'posts' => $match->posts, 'newpost' => $newpost] ); ?>
				</div>
			</div>
		</div>
		<div class="col-md-12 col-sm-12">
			<?= $this->element( 'modtable', array( 'mods' => $match->mods )); ?>
		</div>
	</div>
	<div style="width:300px" class="remodal" id="swm" data-remodal-id="scheduleweek">
		<button data-remodal-action="close" class="remodal-close"></button>
				<div class="col-lg-8 col-md-8 col-sm-8 nopad">
					<?= $this->Form->input( 'day', [ 'id' => 'modalday', 'empty' => 'day', 'label' => false, 'options' => [
						0 => 'Sunday', 1 => 'Monday', 2 => 'Tuesday', 3 => 'Wednesday',
						4 => 'Thursday', 5 => 'Friday', 6 => 'Saturday']] ); ?>
				</div>
				<div class="col-lg-4 col-md-4 col-sm-4 nopad">
					<?= $this->Form->input( 'hour', [ 'id' => 'modalhour', 'empty' => 'hour', 'label' => false, 'options' => [ 
						0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 
						13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23 ]] ); ?>
				</div>
		<button data-remodal-action="cancel" class="remodal-cancel">Cancel</button>
		<button data-remodal-action="confirm" class="remodal-confirm">Apply</button>
	</div>
	<div style="width:200px" class="remodal" id="sim" data-remodal-id="scheduleinterval">
		<button data-remodal-action="close" class="remodal-close"></button>
				<div class="col-lg-6 col-md-6 col-sm-6 nopad">
					<?= $this->Form->input( 'hour', ['id' => 'modalinthour'] ); ?>
				</div>
				<div class="col-lg-6 col-md-6 col-sm-6 nopad">
					<?= $this->Form->input( 'minute', ['id' => 'modalintminute'] ); ?>
				</div>
		<button data-remodal-action="cancel" class="remodal-cancel">Cancel</button>
		<button data-remodal-action="confirm" class="remodal-confirm">Apply</button>
	</div>
</div>
<div class="remodal" style="width:600px" data-remodal-id="settings-modal">
	<div id="progress-controls">
		<div class="form-group">
			<h5>Match Status Controls</h5>
			<button type="button" id="finishbutton" class="btn btn-default">End Match</button>
			<button type="button" id="unstartbutton" class="btn btn-default">Unstart Match</button>
		</div>
		<?= $this->Form->create($match, ['id' => 'intervalform', 'url' => ['action' => 'hostinterval']]); ?>
		<div class="form-inline">
			<h5>Schedule by Host Interval</h5>
			<input type="text" class="form-control" placeholder="Hours" name="hour"></input>
			<div class="input-group">
				<input type="text" class="form-control" placeholder="Minutes" name="minute"></input>
				<span class="input-group-btn">
					<button class="btn btn-default" type="submit">Submit</button>
				</span>
			</div>
		</div>
		<?= $this->Form->end(); ?>
		<?= $this->Form->create($match, ['id' => 'computerform', 'url' => ['action' => 'markcomputer']]); ?>
	</div>
	<div id="lobby-controls">
		<div class="form-inline">
			<h5>Add AI</h5>
			<div class="input-group">
				<input type="hidden" name="id" value="<?=$match->id?>"></hidden>
				<?= $this->Form->select('nation_id', $allnations, ['id'=>'aination','class'=>'form-control', 'empty' => 'Set Nation AI']); ?>
				<span class="input-group-btn">
					<button class="btn btn-default" type="submit">Submit</button>
				</span>
			</div>
		</div>
		<?= $this->Form->end(); ?>
		<h5>Match Status Controls</h5>
		<button type="button" id="cancelbutton" class="btn btn-default">Cancel Match</button>
	</div>
</div>
