<div class="matches view large-12 medium-8 columns content">
    <h3><?= h($match->name) ?></h3>
	<div class="wrapper">
		<div class="maptable">
			<table class="vertical-table" style="background:#fafafa" > <tr>
					<th><?= __('Thrones(Points To Win)') ?></th>
					<td><?= $match->thrones ?>
				<tr>
					<th><?= __('Status') ?></th>
					<td><?= $match::statuses( $match->status ) ?></td>
				</tr>
				<tr>
					<th><?= __('Age') ?></th>
					<td><?= $match::ages( $match->age ) ?></td>
				</tr>
				<tr>
					<th><?= __('Address') ?></th>
					<td><?= $match->address ?></td>
				<tr>
					<th><?= __('Nations') ?></th>
					<td>
						<?php foreach ($match->nations as $nation): ?>
							<?= $nation['name']. ', ' .$nation['subtitle'] ?>
							<?php if( $match->status !== 3 ) { ?>
								<?php if( $nation->_joinData->markdelete == 0 ) { ?>
									<?= $this->Html->link(__('Remove'), ['controller' => 'Matches', 'action' => 'removePlayer', $nation->_joinData->id ]) ?>
								<?php } else { ?>
									<?= "<b>(Removing..)</b>" ?>
							<?php } ?>
						<?php } ?> <br /> <?php endforeach; ?>
					</td>
				</tr>
			</table>
			<?= $this->element( 'modtable', array( 'mods' => $match->mods )); ?>
		</div>
		<div class="mappreview">
			<?= $match->has('map') ? $this->Html->link(
				$this->Html->image( 'maps/' . $match->map->id . '/thumb512.jpeg', ['alt' => 'CakePHP' ]), 
				['controller' => 'Maps', 'action' => 'view', $match->map->id], 
				['escape' => false]) : '' ?>
		</div>
	</div>
</div>
