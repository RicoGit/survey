$(document).ready(function() {

	function ajaxCallRequest(f_method, f_url, f_data) {
		$("#dataSent").val(JSON.stringify(f_data));
		var f_contentType = 'application/json; charset=UTF-8';
		$.ajax({
			url: f_url,
			type: f_method,
			contentType: f_contentType,
			dataType: 'json',
			data: JSON.stringify(f_data),
			success: function(data) {
				console.log("response: " + data);
				var jsonResult = JSON.stringify(data);
				$("#results").val(unescape(jsonResult));
			}
		});
	}

	//
	// Sending form to server
    //
	$("#sendTreeJSon").click(function(event) {
		event.preventDefault();
		var form = $(this).parents('form');
		var method = form.attr('method');
		var url = form.attr('action');
		var jsonData = $(form).serializeObject();
		console.log(JSON.stringify(jsonData));
		ajaxCallRequest(method, url, jsonData);
	});

	$.mockjax({
		url: '/ajaxRequest/treejson/',
		type: 'POST',
		contentType: 'text/json',
		responseTime: 0,
		response: function(settings) {
			var data = settings.data;
			this.responseText = data;
		}
	});

	//
	// Fill form with defaults
	//
	$("#defaultData").click(function(event) {
		event.preventDefault();
		$('#firstname').val('Константин');
		$('#lastname').val('Филимонов');
		$('#middlename').val('Давидович');
		$('#address_street').val('ул. Карла Маркса');
		$('#address_city').val('Самара');
		$('#address_building').val('17');
		$('#address_room').val('42');
		$("[name='email']").val('superintendencia@cia.es');
	});

	// todo generate survey by json from server
	function generate(json) {
		console.log(json)
		// json.forEach(function(node) {
		//
		//
		// })
	}

	//
	// New survey (add remove new question)
	//
	$(document).on('click', '.btn-add', function(event) {
		event.preventDefault();
		var controlForm = $('.controls');
		var currentEntry = $(this).parents('.entry:first');
		var newEntry = $(currentEntry.clone()).appendTo(controlForm);
		newEntry.find('input').val('');
		controlForm.find('.entry:not(:last) .btn-add')
			.removeClass('btn-add').addClass('btn-remove')
			.removeClass('btn-success').addClass('btn-danger')
			.html('<span class="glyphicon glyphicon-minus"></span>');

		var entryIdx = $('.controls .entry').size() - 1;

		console.log('number of entries ' + entryIdx)

		$.each($('.controls .entry:last .form-control'), function(index, item) {
			item.name = 'questions[' + entryIdx + '][' +  $(item).attr("data") + ']';
		});
	});

	$(document).on('click', '.btn-remove', function(event) {
		event.preventDefault();
		$(this).parents('.entry:first').remove();

		$.each($('.controls .entry'), function(index, _item) {
			$.each($(_item).find('.form-control'), function(_index, item) {
				console.log(item)
				item.name = 'questions[' + index + '][' +  $(item).attr("data") + ']';
			})

		});
	});

});

