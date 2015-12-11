<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('List Maps'), ['action' => 'index']) ?></li>
    </ul>
</nav>
<div class="maps form large-9 medium-8 columns content">
    <?= $this->Form->create($map, ['type'=>'file']) ?>
    <fieldset>
		<legend><?= __('Add Map') ?></legend>
		<?= $this->Form->input('Definition', ['type'=>'file']); ?>
		<?= $this->Form->input('Image', ['type'=>'file']); ?>
    </fieldset>
    <?= $this->Form->button(__('Submit')) ?>
    <?= $this->Form->end() ?>
</div>
