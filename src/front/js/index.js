$(document).ready(function() {

	function ajaxCallRequest(f_method, f_url, f_data) {
		$("#dataSent").val(unescape(f_data));
		var f_contentType = 'application/x-www-form-urlencoded; charset=UTF-8';
		$.ajax({
			url: f_url,
			type: f_method,
			contentType: f_contentType,
			dataType: 'json',
			data: f_data,
			success: function(data) {
				var jsonResult = JSON.stringify(data);
				$("#results").val(unescape(jsonResult));
			}
		});
	}

	//
	// Sending form to server
    //
	$("#sendPlainJSon").click(function(event) {
		event.preventDefault();
		var form = $('#surveyForm');
		var method = form.attr('method');
		var url = form.attr('action'); // send to /survey
		var jsonData = {};
		$.each($(form).serializeArray(), function() {
			jsonData[this.name] = this.value;
		});
		var data = JSON.stringify(jsonData);
		console.log(data);
		ajaxCallRequest(method, url, data);
	});
	// $.mockjax({
	// 	url: '/ajaxRequest/plainjson/',
	// 	type: 'POST',
	// 	contentType: 'text/json',
	// 	responseTime: 0,
	// 	response: function(settings) {
	// 		var data = settings.data;
	// 		this.responseText = data;
	// 	}
	// });


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

});

