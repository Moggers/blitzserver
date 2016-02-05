<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
    </ul>
</nav>
<div class="matches view large-9 medium-8 columns content">
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
						<?php foreach ($match->getNations() as $nation): ?>
							<?= $nation['name'] ?>
						<?php if( ($nation['id'] & $match->deleteplayerstring) == 0 ) { ?>
							<?= $this->Html->link(__('Remove'), ['controller' => 'Matches', 'action' => 'removePlayer', $match->id, $nation['id'] ]) ?> <br />
						<?php } else { ?>
							<?= "<b>(Removing..)</b>" ?> <br />
						 <?php } endforeach; ?>
					</td>
				</tr>
			</table>
		</div>
		<div class="mappreview">
			<?= $match->has('map') ? $this->Html->link(
				$this->Html->image( 'maps/' . $match->map->id . '/thumb512.jpeg', ['alt' => 'CakePHP' ]), 
				['controller' => 'Maps', 'action' => 'view', $match->map->id], 
				['escape' => false]) : '' ?>
		</div>
	</div>
</div>
