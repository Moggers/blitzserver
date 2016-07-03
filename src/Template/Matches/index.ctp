<script type='text/javascript' >
	var waitExtend = 5;
	var refreshmeme = 5;
	function refresh()
	{
		$('#refresh').addClass('animate-spin');
		$.get( window.location.pathname + '?layout=false', function( data, res ) {  
			$(".matches tabs" ).html($("table", data));
			refreshmeme = waitExtend;
			$('#refresh').removeClass('animate-spin');
		});
	}
	$(document).ready( function() {
		$('.refresh-btn').on('click', function(e) {
			refresh();
		});
	} );
</script>
<div class="matches index col-md-12 col-sm-12">
    <h3><?= __('Dominions 4 Matches') ?></h3>
	<button class="btn refresh-btn btn-default btn-small" style="float:right"><a class="glyphicon glyphicon-refresh" id="refresh" ></a></button>
	<?= $this->element( 'matchtable', [ 'progressmatches' => $progressmatches, 'finishedmatches' => $finishedmatches, 'lobbymatches' => $lobbymatches] ); ?>
</div>
