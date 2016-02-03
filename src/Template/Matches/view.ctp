<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('Edit Match'), ['action' => 'edit', $match->id]) ?> </li>
        <li><?= $this->Form->postLink(__('Delete Match'), ['action' => 'delete', $match->id], ['confirm' => __('Are you sure you want to delete # {0}?', $match->id)]) ?> </li>
        <li><?= $this->Html->link(__('List Matches'), ['action' => 'index']) ?> </li>
        <li><?= $this->Html->link(__('New Match'), ['action' => 'add']) ?> </li>
        <li><?= $this->Html->link(__('List Maps'), ['controller' => 'Maps', 'action' => 'index']) ?> </li>
        <li><?= $this->Html->link(__('New Map'), ['controller' => 'Maps', 'action' => 'add']) ?> </li>
    </ul>
</nav>
<div class="matches view large-9 medium-8 columns content">
    <h3><?= h($match->name) ?></h3>
	<table class="vertical-table" style="background:#fafafa;" >
		<tr>
			<td style="vertical-align: top">
				<table class="vertical-table" style="background:#fafafa">
					<tr>
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
						<th><?= __('Nations') ?></th>
						<td>
							<?php foreach ($match::getNations( $match->playerstring) as $race): ?>
								<?= $race ?> <br />
							<?php endforeach; ?>
						</td>
					</tr>
				</table>
			</td>
			<td><?= $match->has('map') ? $this->Html->link(
				$this->Html->image( 'maps/' . $match->map->id . '/thumb512.jpeg', ['alt' => 'CakePHP' ]), 
				['controller' => 'Maps', 'action' => 'view', $match->map->id], 
				['escape' => false]) : '' ?> </td>
		</tr>
	</table>
</div>
