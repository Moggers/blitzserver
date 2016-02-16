<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('New Mod'), ['action' => 'add']) ?></li>
    </ul>
</nav>
<div class="mods index large-9 medium-8 columns content">
    <h3><?= __('Mods') ?></h3>
	<?= $this->element( 'modtable', array( 'mods' => $mods ) ); ?>
</div>
