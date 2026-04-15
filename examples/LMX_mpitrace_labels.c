/* Copyright 2026 Lenovo and C. Pospiech
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * File: LMX_mpitrace_labels.c
 * Description: src/features/basic/LMX_mpitrace_labels.c
 * Author: Dr. Christoph Pospiech
 */

#include <LMX_log_and_collect.h>

/**
 * These initial values were created by applying the perl script
 * [create_labels.pl](@ref interception) to
 * a text version of http://mpi-forum.org/docs/mpi-3.0/mpi30-report.pdf
 */
char *LMX_mpi_label[number_mpi_ids] = {
    /*
     * Chapter 1 Introduction to MPI
     *
     * Chapter 2 MPI Terms and Conventions
     *
     * Chapter 3 Point-to-Point Communication
     */
    "MPI_Send", "MPI_Recv", "MPI_Get_count", "MPI_Bsend", "MPI_Ssend", "MPI_Rsend", "MPI_Buffer_attach",
    "MPI_Buffer_detach", "MPI_Isend", "MPI_Ibsend", "MPI_Issend", "MPI_Irsend", "MPI_Irecv", "MPI_Wait", "MPI_Test",
    "MPI_Request_free", "MPI_Waitany", "MPI_Testany", "MPI_Waitall", "MPI_Testall", "MPI_Waitsome", "MPI_Testsome",
    "MPI_Request_get_status", "MPI_Iprobe", "MPI_Probe", "MPI_Improbe", "MPI_Mprobe", "MPI_Mrecv", "MPI_Imrecv",
    "MPI_Cancel", "MPI_Test_cancelled", "MPI_Send_init", "MPI_Bsend_init", "MPI_Ssend_init", "MPI_Rsend_init",
    "MPI_Recv_init", "MPI_Start", "MPI_Startall", "MPI_Sendrecv", "MPI_Sendrecv_replace",
    /*
     * Chapter 4 Datatypes
     */
    "MPI_Type_contiguous", "MPI_Type_vector", "MPI_Type_create_hvector", "MPI_Type_indexed", "MPI_Type_create_hindexed",
    "MPI_Type_create_indexed_block", "MPI_Type_create_hindexed_block", "MPI_Type_create_struct",
    "MPI_Type_create_subarray", "MPI_Type_create_darray", "MPI_Get_address", "MPI_Type_size", "MPI_Type_size_x",
    "MPI_Type_get_extent", "MPI_Type_get_extent_x", "MPI_Type_create_resized", "MPI_Type_get_true_extent",
    "MPI_Type_get_true_extent_x", "MPI_Type_commit", "MPI_Type_free", "MPI_Type_dup", "MPI_Get_elements",
    "MPI_Get_elements_x", "MPI_Type_get_envelope", "MPI_Type_get_contents", "MPI_Pack", "MPI_Unpack", "MPI_Pack_size",
    "MPI_Pack_external", "MPI_Unpack_external", "MPI_Pack_external_size",
    /*
     * Chapter 5 Collective Communication
     */
    "MPI_Barrier", "MPI_Bcast", "MPI_Gather", "MPI_Gatherv", "MPI_Scatter", "MPI_Scatterv", "MPI_Allgather",
    "MPI_Allgatherv", "MPI_Alltoall", "MPI_Alltoallv", "MPI_Alltoallw", "MPI_Reduce", "MPI_Op_create", "MPI_Op_free",
    "MPI_Allreduce", "MPI_Reduce_local", "MPI_Op_commutative", "MPI_Reduce_scatter_block", "MPI_Reduce_scatter",
    "MPI_Scan", "MPI_Exscan", "MPI_Ibarrier", "MPI_Ibcast", "MPI_Igather", "MPI_Igatherv", "MPI_Iscatter",
    "MPI_Iscatterv", "MPI_Iallgather", "MPI_Iallgatherv", "MPI_Ialltoall", "MPI_Ialltoallv", "MPI_Ialltoallw",
    "MPI_Ireduce", "MPI_Iallreduce", "MPI_Ireduce_scatter_block", "MPI_Ireduce_scatter", "MPI_Iscan", "MPI_Iexscan",
    /*
     * Chapter 6 Groups, Contexts, Communicators,
     */
    "MPI_Group_size", "MPI_Group_rank", "MPI_Group_translate_ranks", "MPI_Group_compare", "MPI_Comm_group",
    "MPI_Group_union", "MPI_Group_intersection", "MPI_Group_difference", "MPI_Group_incl", "MPI_Group_excl",
    "MPI_Group_range_incl", "MPI_Group_range_excl", "MPI_Group_free", "MPI_Comm_size", "MPI_Comm_rank",
    "MPI_Comm_compare", "MPI_Comm_dup", "MPI_Comm_dup_with_info", "MPI_Comm_idup", "MPI_Comm_create",
    "MPI_Comm_create_group", "MPI_Comm_split", "MPI_Comm_split_type", "MPI_Comm_free", "MPI_Comm_set_info",
    "MPI_Comm_get_info", "MPI_Comm_test_inter", "MPI_Comm_remote_size", "MPI_Comm_remote_group", "MPI_Intercomm_create",
    "MPI_Intercomm_merge", "MPI_Comm_create_keyval", "MPI_Comm_free_keyval", "MPI_Comm_set_attr", "MPI_Comm_get_attr",
    "MPI_Comm_delete_attr", "MPI_Win_create_keyval", "MPI_Win_free_keyval", "MPI_Win_set_attr", "MPI_Win_get_attr",
    "MPI_Win_delete_attr", "MPI_Type_create_keyval", "MPI_Type_free_keyval", "MPI_Type_set_attr", "MPI_Type_get_attr",
    "MPI_Type_delete_attr", "MPI_Comm_set_name", "MPI_Comm_get_name", "MPI_Type_set_name", "MPI_Type_get_name",
    "MPI_Win_set_name", "MPI_Win_get_name",
    /*
     * Chapter 7 Process Topologies
     */
    "MPI_Cart_create", "MPI_Dims_create", "MPI_Graph_create", "MPI_Dist_graph_create_adjacent", "MPI_Dist_graph_create",
    "MPI_Topo_test", "MPI_Graphdims_get", "MPI_Graph_get", "MPI_Cartdim_get", "MPI_Cart_get", "MPI_Cart_rank",
    "MPI_Cart_coords", "MPI_Graph_neighbors_count", "MPI_Graph_neighbors", "MPI_Dist_graph_neighbors_count",
    "MPI_Dist_graph_neighbors", "MPI_Cart_shift", "MPI_Cart_sub", "MPI_Cart_map", "MPI_Graph_map",
    "MPI_Neighbor_allgather", "MPI_Neighbor_allgatherv", "MPI_Neighbor_alltoall", "MPI_Neighbor_alltoallv",
    "MPI_Neighbor_alltoallw", "MPI_Ineighbor_allgather", "MPI_Ineighbor_allgatherv", "MPI_Ineighbor_alltoall",
    "MPI_Ineighbor_alltoallv", "MPI_Ineighbor_alltoallw",
    /*
     * Chapter 8 MPI Environmental Management
     */
    "MPI_Get_version", "MPI_Get_library_version", "MPI_Get_processor_name", "MPI_Alloc_mem", "MPI_Free_mem",
    "MPI_Comm_create_errhandler", "MPI_Comm_set_errhandler", "MPI_Comm_get_errhandler", "MPI_Win_create_errhandler",
    "MPI_Win_set_errhandler", "MPI_Win_get_errhandler", "MPI_File_create_errhandler", "MPI_File_set_errhandler",
    "MPI_File_get_errhandler", "MPI_Errhandler_free", "MPI_Error_string", "MPI_Error_class", "MPI_Add_error_class",
    "MPI_Add_error_code", "MPI_Add_error_string", "MPI_Comm_call_errhandler", "MPI_Win_call_errhandler",
    "MPI_File_call_errhandler", "MPI_Initialized", "MPI_Abort", "MPI_Finalized",
    /*
     * Chapter 9 The Info Object
     */
    "MPI_Info_create", "MPI_Info_set", "MPI_Info_delete", "MPI_Info_get", "MPI_Info_get_valuelen", "MPI_Info_get_nkeys",
    "MPI_Info_get_nthkey", "MPI_Info_dup", "MPI_Info_free",
    /*
     * Chapter 10 Process Creation and Management
     */
    "MPI_Comm_spawn", "MPI_Comm_get_parent", "MPI_Comm_spawn_multiple", "MPI_Open_port", "MPI_Close_port",
    "MPI_Comm_accept", "MPI_Comm_connect", "MPI_Publish_name", "MPI_Unpublish_name", "MPI_Lookup_name",
    "MPI_Comm_disconnect", "MPI_Comm_join",
    /*
     * Chapter 11 One-Sided Communications
     */
    "MPI_Win_create", "MPI_Win_allocate", "MPI_Win_allocate_shared", "MPI_Win_shared_query", "MPI_Win_create_dynamic",
    "MPI_Win_attach", "MPI_Win_detach", "MPI_Win_free", "MPI_Win_get_group", "MPI_Win_set_info", "MPI_Win_get_info",
    "MPI_Put", "MPI_Get", "MPI_Accumulate", "MPI_Get_accumulate", "MPI_Fetch_and_op", "MPI_Compare_and_swap",
    "MPI_Rput", "MPI_Rget", "MPI_Raccumulate", "MPI_Rget_accumulate", "MPI_Win_fence", "MPI_Win_start",
    "MPI_Win_complete", "MPI_Win_post", "MPI_Win_wait", "MPI_Win_test", "MPI_Win_lock", "MPI_Win_lock_all",
    "MPI_Win_unlock", "MPI_Win_unlock_all", "MPI_Win_flush", "MPI_Win_flush_all", "MPI_Win_flush_local",
    "MPI_Win_flush_local_all", "MPI_Win_sync",
    /*
     * Chapter 12 External Interfaces
     */
    "MPI_Grequest_start", "MPI_Grequest_complete", "MPI_Status_set_elements", "MPI_Status_set_elements_x",
    "MPI_Status_set_cancelled", "MPI_Query_thread", "MPI_Is_thread_main",
    /*
     * Chapter 13 I/O
     */
    "MPI_File_open", "MPI_File_close", "MPI_File_delete", "MPI_File_set_size", "MPI_File_preallocate",
    "MPI_File_get_size", "MPI_File_get_group", "MPI_File_get_amode", "MPI_File_set_info", "MPI_File_get_info",
    "MPI_File_set_view", "MPI_File_get_view", "MPI_File_read_at", "MPI_File_read_at_all", "MPI_File_write_at",
    "MPI_File_write_at_all", "MPI_File_iread_at", "MPI_File_iwrite_at", "MPI_File_read", "MPI_File_read_all",
    "MPI_File_write", "MPI_File_write_all", "MPI_File_iread", "MPI_File_iwrite", "MPI_File_seek",
    "MPI_File_get_position", "MPI_File_get_byte_offset", "MPI_File_read_shared", "MPI_File_write_shared",
    "MPI_File_iread_shared", "MPI_File_iwrite_shared", "MPI_File_read_ordered", "MPI_File_write_ordered",
    "MPI_File_seek_shared", "MPI_File_get_position_shared", "MPI_File_read_at_all_begin", "MPI_File_read_at_all_end",
    "MPI_File_write_at_all_begin", "MPI_File_write_at_all_end", "MPI_File_read_all_begin", "MPI_File_read_all_end",
    "MPI_File_write_all_begin", "MPI_File_write_all_end", "MPI_File_read_ordered_begin", "MPI_File_read_ordered_end",
    "MPI_File_write_ordered_begin", "MPI_File_write_ordered_end", "MPI_File_get_type_extent", "MPI_Register_datarep",
    "MPI_File_set_atomicity", "MPI_File_get_atomicity", "MPI_File_sync",
    /*
     * Chapter 15 Deprecated Functions
     */
    "MPI_Keyval_create", "MPI_Keyval_free", "MPI_Attr_put", "MPI_Attr_get", "MPI_Attr_delete",
    /*
     * Chapter 16 Removed Interfaces
     *
     * Chapter 17 Language Bindings
     */
    "MPI_Type_create_f90_real", "MPI_Type_create_f90_complex", "MPI_Type_create_f90_integer", "MPI_Type_match_size",
    "MPI_Status_f2c", "MPI_Status_c2f", "MPI_Status_f082c", "MPI_Status_c2f08", "MPI_Status_f2f08", "MPI_Status_f082f",
    "MPI_Number_calls",
    /*
     * Created 349 labels
     *
     * Identifiers for load imbalance detection
     */
    "MPI_Barrier_barrier", "MPI_Bcast_barrier", "MPI_Gather_barrier", "MPI_Gatherv_barrier", "MPI_Scatter_barrier",
    "MPI_Scatterv_barrier", "MPI_Allgather_barrier", "MPI_Allgatherv_barrier", "MPI_Alltoall_barrier",
    "MPI_Alltoallv_barrier", "MPI_Alltoallw_barrier", "MPI_Reduce_barrier", "MPI_Allreduce_barrier",
    "MPI_Reduce_scatter_block_barrier", "MPI_Reduce_scatter_barrier", "MPI_Scan_barrier", "MPI_Exscan_barrier",
    "MPI_Ibarrier_barrier", "MPI_Ibcast_barrier", "MPI_Igather_barrier", "MPI_Igatherv_barrier", "MPI_Iscatter_barrier",
    "MPI_Iscatterv_barrier", "MPI_Iallgather_barrier", "MPI_Iallgatherv_barrier", "MPI_Ialltoall_barrier",
    "MPI_Ialltoallv_barrier", "MPI_Ialltoallw_barrier", "MPI_Ireduce_barrier", "MPI_Iallreduce_barrier",
    "MPI_Ireduce_scatter_block_barrier", "MPI_Ireduce_scatter_barrier", "MPI_Iscan_barrier", "MPI_Iexscan_barrier",
    /*
     *  Created 383 labels
     */
};
