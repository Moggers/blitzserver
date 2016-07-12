$(document).on('ready', function(e) {
	$('#progress-controls').hide();
	$('#lobby-controls').hide();
	if( matchstatus == 3 ) {
		$('#progress-controls').show();
	} else if( matchstatus == 1 || matchstatus == 0 ) {
		$('#lobby-controls').show();
	}
	$('#cancelbutton').on('click', function(e) {
		$.get('/matches/destroy/'+matchid, function(data) {
			let json = JSON.parse(data);
			if( json.status == 0 ) {
				alert("Successfully canceled game, redirecting to index");
				window.location = "/";
			} else if( json.status == 1 ) {
				alert("Incorrect Password");
			} else {
				alert("Unknown failure, please file a bug report" );
			}
		});
	});
	$('#finishbutton').on('click', function(e) {
		$.get('/matches/finish/'+matchid, function(data) {
			let json = JSON.parse(data);
			if( json.status == 0 ) {
				alert("Successfully ended game, refresh to see changes");
			} else if( json.status == 1 ) {
				alert("Incorrect Password");
			} else {
				alert("Unknown failure, please file a bug report" );
			}
		});
	});
	$('#unstartbutton').on('click', function(e) {
		$.get('/matches/unstart/'+matchid, function(data) {
			let json = JSON.parse(data);
			if( json.status == 0 ) {
				alert("Successfully unstarted game, refresh to see changes");
			} else if( json.status == 1 ) {
				alert("Incorrect Password");
			} else {
				alert("Unknown failure, please file a bug report" );
			}
		});
	});
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
		var data = JSON.parse( res );
		if( data.status == 1 ) {
			window.location = '/matches/view/'+data.id;
		} else {
			window.location.hash = "modal";
		}
	});
	$('#computerform').ajaxForm( function(res) {
		var data= JSON.parse(res);
		if( data.status == 0 ) {
			alert("Success");
		} else {
			alert("Failure");
		}
	});

	$('#emailform').ajaxForm( function(res) {
		console.log( "wew" );
		var data = JSON.parse(res);
		if( data.status == 0 ) {
			alert( "Roger!");
		} else {
			alert( "OH NOOO");
		}
	});
	if( $('#localtime').length ) {
		var time = moment.tz($('#localtime')[0].innerHTML, 'YYYY-MM-DD HH:nm:ss', 'Europe/Dublin').tz( moment.tz.guess())
		$('#localtime')[0].innerHTML = moment.preciseDiff( moment(), time);
	}
	$('#turndelay').on('change', function(e)
	{
		$.post('../turndelay', {'id': matchid, 'turndelay': $('#turndelay').val()}, function(res) {
			var data = JSON.parse(res);
			if( data.status == 0 ){
				alert('Done');
			} if( data.status == 1 ) {
				window.location.hash = "modal";
			} if( data.status == 2 ){
				alert('Unknown error');
			}
		});
	});
});
