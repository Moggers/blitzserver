<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('List Mods'), ['action' => 'index']) ?></li>
    </ul>
</nav>
<div class="mods form large-9 medium-8 columns content">
    <?= $this->Form->create($mod, ['type'=>'file']) ?>
    <fieldset>
        <legend><?= __('Add Mod') ?></legend>
		<?= $this->Form->input('Archive', [ 'label' => 'Zip Archive', 'type'=>'file', 'accept'=>'.zip,.7z,.rar']); ?>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
