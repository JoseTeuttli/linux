#include "hyperv_vmbus.h"
#include "hv_utils_transport.h"
//this is so bindgen generates a binding for this, might want to add name later, currently can be accessed as struct _bindgen_ty_101
static struct fcopy_transaction_struct{
	int state;   /* hvutil_device_state */
	int recv_len; /* number of bytes received. */
	struct hv_fcopy_hdr  *fcopy_msg; /* current message */
	struct vmbus_channel *recv_channel; /* chn we got the request */
	u64 recv_req_id; /* request ID. */
};