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
extern struct delayed_work fcopy_timeout_work;
extern struct work_struct fcopy_send_work;
extern struct fcopy_transaction_struct fcopy_transaction;
extern int dm_reg_value;
extern struct hvutil_transport *hvt;
extern u8 *recv_buffer;
extern char fcopy_devname[14];
#define FCOPY_VER_COUNT 1
#define FW_VER_COUNT 1