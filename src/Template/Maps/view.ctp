<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
    </ul>
</nav>
<div class="maps view large-9 medium-8 columns content">
    <h3><?= h($map->name) ?></h3>
	<div class="wrapper">
		<div class="maptable">
			<table class="vertical-table" style="background:#fafafa">
				<td style="vertical-align: top">
				<table class="vertical-table" style="background:#fafafa">
					<tr>
						<th><?= __('Description') ?></th>
						<td><?= h($map->description) ?></td>
					</tr>
					<tr>
						<th><?= __('Provinces(Sea Provinces)')?> </th>
						<td><?= h($map->prov.'('.$map->seaprov) .')' ?> </td>
					</tr>
					<tr>
					</tr>
				</table>
				</td>
			</table>
		</div>
		<div class="mappreview">
			<td><?=$this->Html->image( 'maps/' . $map->id . '/thumb512.jpeg', ['alt' => 'CakePHP' ]) ?></td>
		</div>
	</div>
	</table>
</div>
