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
			console.log( 'wake me up' );
			var data = JSON.parse( res );
			if( data.status == 1 ) {
				window.location = '/matches/view/'+data.id;
			} else {
				window.location.hash = "modal";
			}
		});
		$('#localtime')[0].innerHTML = moment.tz($('#localtime')[0].innerHTML, 'YYYY-MM-DD HH:nm:ss', 'Europe/Dublin').tz( moment.tz.guess()).fromNow();
	});
</script>

<div class="matches view large-12 columns content">
    <h3><?= h($match->name) ?></h3>
	<div class="wrapper">
		<div class="maptable">
			<table class="vertical-table" style="background:#fafafa" >
				<tr>
					<th><?= __('Thrones(Points To Win)') ?></th>
					<td><?= $match->thrones ?>
				<tr>
					<th><?= __('Age') ?></th>
					<td><?= $match::ages( $match->age ) ?></td>
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
			<div class="large-6 medium-12 small-12 columns content">
				<table class="vertical-table" style="background:#fafafa">
				<th>Players</th><th>Actions</th>
				<?php foreach ($match->nations as $nation): ?>
					<tr>
						<td>
						<?=$nation['name']. ', ' .$nation['subtitle']?>
						</td>
						<?php if( $match->status !== 3 ) { ?>
							<?php if( $nation->_joinData->markdelete == 0 ) { ?>
								<td>
								<?= $this->Html->link(__('Remove'), ['controller' => 'Matches', 'action' => 'removePlayer', $nation->_joinData->id ]) ?>
								</td>
							<?php } else { ?>
								<?= "<b>(Removing..)</b>" ?>
							<?php } ?>
						</tr>
					<?php } ?> <br /> <?php endforeach; ?>
				</table>
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
			</div>
		</div>
		<div class="mappreview">
			<?= $match->has('map') ? $this->Html->link(
				$this->Html->image( 'maps/' . $match->map->id . '/thumb1024.jpeg', ['alt' => 'CakePHP' ]), 
				['controller' => 'Maps', 'action' => 'view', $match->map->id], 
				['escape' => false]) : '' ?>
		</div>
		<div class="mods index large-12 medium-12 columns content">
			<?= $this->element( 'modtable', array( 'mods' => $match->mods )); ?>
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

